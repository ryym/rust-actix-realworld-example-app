use mdl;

#[derive(Debug, Serialize)]
pub struct User {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl User {
    pub fn from_model(token: String, user: mdl::User) -> User {
        User {
            token,
            username: user.username,
            email: user.email,
            bio: user.bio,
            image: user.image,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Debug, Serialize)]
pub struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

impl Profile {
    pub fn from_user(user: mdl::User, following: bool) -> Profile {
        Profile {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub profile: Profile,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub favorited: bool,
    pub favorites_count: u32,
    pub author: Profile,
}

#[derive(Debug, Serialize)]
pub struct ArticleResponse {
    pub article: Article,
}
