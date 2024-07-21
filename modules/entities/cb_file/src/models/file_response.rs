use rocket::{
    http::{ContentType, Header},
    response::Responder,
    Request, Response,
};
pub struct FileDownloadResponse {
    pub file: rocket::fs::NamedFile,
    pub file_name: String,
    pub content_type: ContentType,
}
impl<'r> Responder<'r, 'static> for FileDownloadResponse {
    fn respond_to(self, req: &'r Request<'_>) -> Result<Response<'static>, rocket::http::Status> {
        let mut response = self.file.respond_to(req)?;
        response.set_header(ContentType::Binary);
        response.set_header(Header::new(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", self.file_name),
        ));
        Ok(response)
    }
}
pub struct FileRetrieveResponse {
    pub file: rocket::fs::NamedFile,
    pub file_name: String,
    pub content_type: ContentType,
}
impl<'r> Responder<'r, 'static> for FileRetrieveResponse {
    fn respond_to(self, req: &'r Request<'_>) -> Result<Response<'static>, rocket::http::Status> {
        let mut response = self.file.respond_to(req)?;
        response.set_header(self.content_type.clone());
        response.set_header(Header::new(
            "Content-Disposition",
            format!("inline; filename=\"{}\"", self.file_name),
        ));
        Ok(response)
    }
}
