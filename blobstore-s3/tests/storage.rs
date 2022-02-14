use blobstore_s3_lib::StorageClient;
use wasmcloud_interface_blobstore::*;

/// Tests
/// - create_container
/// - remove_container
/// - container_exists
#[tokio::test]
async fn test_create_container() {
    let s3 = StorageClient::async_default().await;
    let ctx = wasmbus_rpc::common::Context::default();

    let num = rand::random::<u64>();
    let bucket = format!("test.{}.hello", num);

    assert_eq!(s3.container_exists(&ctx, &bucket).await, Ok(false));
    s3.create_container(&ctx, &bucket).await.unwrap();

    assert_eq!(s3.container_exists(&ctx, &bucket).await, Ok(true));

    s3.remove_containers(&ctx, &vec![bucket])
        .await
        .expect("remove containers");
}

/// Tests
/// - put_object
/// - remove_object
/// - object_exists
#[tokio::test]
async fn test_create_object() {
    let s3 = StorageClient::async_default().await;
    let ctx = wasmbus_rpc::common::Context::default();

    let num = rand::random::<u64>();
    let bucket = format!("test.{}.hello", num);

    s3.create_container(&ctx, &bucket).await.unwrap();

    let object_bytes = b"hello-world!".to_vec();
    let resp = s3
        .put_object(
            &ctx,
            &PutObjectRequest {
                chunk: Chunk {
                    bytes: object_bytes.clone(),
                    container_id: bucket.clone(),
                    is_last: true,
                    object_id: "object.1".to_string(),
                    offset: 0,
                },
                content_encoding: None,
                content_type: None,
            },
        )
        .await
        .expect("put object");
    assert_eq!(resp.stream_id, None);

    assert_eq!(
        s3.object_exists(
            &ctx,
            &ContainerObject {
                container_id: bucket.clone(),
                object_id: "object.1".to_string(),
            },
        )
        .await,
        Ok(true)
    );

    s3.remove_objects(
        &ctx,
        &RemoveObjectsRequest {
            container_id: bucket.clone(),
            objects: vec!["object.1".to_string()],
        },
    )
    .await
    .expect("remove object");

    assert_eq!(
        s3.object_exists(
            &ctx,
            &ContainerObject {
                container_id: bucket.clone(),
                object_id: "object.1".to_string(),
            },
        )
        .await,
        Ok(false)
    );

    s3.remove_containers(&ctx, &vec![bucket])
        .await
        .expect("remove containers");
}

/// Tests:
/// - list_objects
#[tokio::test]
async fn test_list_objects() {
    let s3 = StorageClient::async_default().await;
    let ctx = wasmbus_rpc::common::Context::default();

    let num = rand::random::<u64>();
    let bucket = format!("test.{}.hello", num);

    s3.create_container(&ctx, &bucket).await.unwrap();

    let req = ListObjectsRequest {
        container_id: bucket.clone(),
        continuation: None,
        end_before: None,
        end_with: None,
        max_items: None,
        start_with: None,
    };
    let objs = s3.list_objects(&ctx, &req).await.expect("list objects");
    assert_eq!(objs.continuation, None);
    assert_eq!(objs.is_last, true);
    assert_eq!(objs.objects.len(), 0);

    let object_bytes = b"hello-world!".to_vec();
    let resp = s3
        .put_object(
            &ctx,
            &PutObjectRequest {
                chunk: Chunk {
                    bytes: object_bytes.clone(),
                    container_id: bucket.clone(),
                    is_last: true,
                    object_id: "object.1".to_string(),
                    offset: 0,
                },
                content_encoding: None,
                content_type: None,
            },
        )
        .await
        .expect("put object");
    assert_eq!(resp.stream_id, None);

    let req = ListObjectsRequest {
        container_id: bucket.clone(),
        continuation: None,
        end_before: None,
        end_with: None,
        max_items: None,
        start_with: None,
    };
    let objs = s3.list_objects(&ctx, &req).await.expect("list objects");
    let meta = objs.objects.get(0).unwrap();
    assert_eq!(&meta.container_id, &bucket);
    assert_eq!(meta.size as usize, object_bytes.len());
    assert_eq!(&meta.object_id, "object.1");

    s3.remove_containers(&ctx, &vec![bucket])
        .await
        .expect("remove containers");
}

/// Tests
/// - get_object_range
#[tokio::test]
async fn test_get_object_range() {
    let s3 = StorageClient::async_default().await;
    let ctx = wasmbus_rpc::common::Context::default();
    let num = rand::random::<u64>();
    let bucket = format!("test.{}.hello", num);

    s3.create_container(&ctx, &bucket).await.unwrap();
    let object_bytes = b"abcdefghijklmnopqrstuvwxyz".to_vec();
    let _ = s3
        .put_object(
            &ctx,
            &PutObjectRequest {
                chunk: Chunk {
                    bytes: object_bytes.clone(),
                    container_id: bucket.clone(),
                    is_last: true,
                    object_id: "object.1".to_string(),
                    offset: 0,
                },
                content_encoding: None,
                content_type: None,
            },
        )
        .await
        .expect("put object");

    let range_mid = s3
        .get_object(
            &ctx,
            &GetObjectRequest {
                container_id: bucket.clone(),
                object_id: "object.1".to_string(),
                range_start: Some(6),
                range_end: Some(12),
                chunk_size: None,
            },
        )
        .await
        .expect("get-object-range-0");
    assert_eq!(range_mid.content_length, 7);
    assert_eq!(
        range_mid.initial_chunk.as_ref().unwrap().bytes,
        b"ghijklm".to_vec()
    );

    // range with omitted end
    let range_to_end = s3
        .get_object(
            &ctx,
            &GetObjectRequest {
                container_id: bucket.clone(),
                object_id: "object.1".to_string(),
                range_start: Some(22),
                range_end: None,
                chunk_size: None,
            },
        )
        .await
        .expect("get-object-range-2");
    assert_eq!(range_to_end.content_length, 4);
    assert_eq!(
        range_to_end.initial_chunk.as_ref().unwrap().bytes,
        b"wxyz".to_vec()
    );

    // range with omitted begin
    let range_from_start = s3
        .get_object(
            &ctx,
            &GetObjectRequest {
                container_id: bucket.clone(),
                object_id: "object.1".to_string(),
                range_start: None,
                range_end: Some(3),
                chunk_size: None,
            },
        )
        .await
        .expect("get-object-range-1");
    assert_eq!(
        range_from_start.initial_chunk.as_ref().unwrap().bytes,
        b"abcd".to_vec()
    );
    //assert_eq!(range_from_start.content_length, 4);

    s3.remove_containers(&ctx, &vec![bucket])
        .await
        .expect("remove containers");
}
