pub struct Forest {
    trees: Vec<Tree>
}

impl Forest {

    pub async fn Create(
        content: String,
        parent: Option<Comment>
    ) -> Result<Comment> {}

    pub async fn Update(
        content: String,
        comment: Comment,
    ) -> Result<Comment> {}

    pub async fn Delete(
        comment: Comment,
    ) -> Result<Comment> {}


}