use anyhow::Result;
use cloud_storage::{Client as GcsClient, Object};

const BUCKET_NAME: &str = "szpp-judge";
const SUBMITS_FOLDER_NAME: &str = "submits";
const TESTCASES_FOLDER_NAME: &str = "testcases";

pub struct Client {
    gcs_client: GcsClient,
}

impl Client {
    pub fn new() -> Self {
        let gcs_client = GcsClient::default();
        Self { gcs_client }
    }

    pub async fn download_submit_source(&self, id: i32) -> Result<Vec<u8>> {
        let path = format!("{}/{}", SUBMITS_FOLDER_NAME, id);
        let bytes = self
            .gcs_client
            .object()
            .download(BUCKET_NAME, &path)
            .await?;
        Ok(bytes)
    }

    pub async fn upload_submit_source(&self, id: i32, submit_source: &str) -> Result<Object> {
        let obj = self
            .gcs_client
            .object()
            .create(
                BUCKET_NAME,
                submit_source.as_bytes().to_vec(),
                format!("{}/{}", SUBMITS_FOLDER_NAME, id).as_str(),
                "plain/text",
            )
            .await?;
        Ok(obj)
    }

    pub async fn download_testcase(&self, id: i32) -> Result<Vec<u8>> {
        let path = format!("{}/{}", TESTCASES_FOLDER_NAME, id);
        let bytes = self
            .gcs_client
            .object()
            .download(BUCKET_NAME, &path)
            .await?;
        Ok(bytes)
    }

    pub async fn upload_testcase(
        &self,
        testcase_id: i32,
        name: &str,
        input: &str,
        output: &str,
    ) -> Result<(Object, Object)> {
        let obj_client = self.gcs_client.object();
        let (in_path, out_path) = (
            format!("{}/{}/in/{}", TESTCASES_FOLDER_NAME, testcase_id, name),
            format!("{}/{}/out/{}", TESTCASES_FOLDER_NAME, testcase_id, name),
        );
        let (obj_in, obj_out) = tokio::join!(
            obj_client.create(
                BUCKET_NAME,
                input.as_bytes().to_vec(),
                in_path.as_str(),
                "plain/text",
            ),
            obj_client.create(
                BUCKET_NAME,
                output.as_bytes().to_vec(),
                out_path.as_str(),
                "plain/text",
            )
        );
        Ok((obj_in?, obj_out?))
    }
}
