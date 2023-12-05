use uuid::UUID;


pub struct Forest {
    trees: Vec<Tree>
}

impl Forest {

    pub async fn Create(
        content: String,
        parent: Option<Comment>
    ) -> Result<Comment, Error> {}

    pub async fn Read(
        id: UUID,
    ) -> Result<Comment, Error> {}

    pub async fn Update(
        content: String,
        comment: Comment,
    ) -> Result<Comment, Error> {}

    pub async fn Delete(
        comment: Comment,
    ) -> Result<Comment, Error> {}


}