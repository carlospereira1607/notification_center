use async_trait::async_trait;

#[async_trait]
trait NotificationApi {
    async fn create(&self);

    async fn get(&self);

    async fn get_all(&self);

    async fn mark_as_seen(&self);

    async fn mark_as_deleted(&self);
}