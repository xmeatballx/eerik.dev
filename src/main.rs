use askama_axum::Template;
use axum::{
    extract::Path, http::StatusCode, response::{Html, IntoResponse, Response}, routing::get, Json, Router
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod content;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_templates=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with some routes
    let app = Router::new()
        .route("/", get(home))
        .route("/project/:selected_index", get(project))
        .route("/featured_posts", get(featured_blog_posts))
        .route("/posts", get(blog))
        .route("/posts/:slug", get(blog_post))
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .with_state(content::project::PROJECTS);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn blog() -> impl IntoResponse {
    let template = content::blog::BlogAllPostsTemplate { posts: &content::blog::BLOG_POSTS };
    HtmlTemplate(template)
}

async fn featured_blog_posts() -> impl IntoResponse {
    let featured_posts: Vec<&content::blog::BlogPost> = content::blog::BLOG_POSTS.iter().filter_map(|&p| if p.featured { Some(p) } else { None }).collect();
    let template = content::blog::FeaturedBlogsTemplate { featured_posts };
    HtmlTemplate(template)
}

async fn blog_post(Path(slug): Path<String>) -> impl IntoResponse {
    let post: &content::blog::BlogPost = &content::blog::BLOG_POSTS.into_iter().find(|&p| p.slug.to_string() == slug).unwrap();
    let html = markdown::to_html(&std::fs::read_to_string("./assets/markdown/".to_owned()+post.markdown_path).expect("No file by this name"));
    let template = content::blog::BlogPostTemplate { post, html };
    HtmlTemplate(template)
}

async fn home(axum::extract::State(projects): axum::extract::State<content::project::Projects>) -> impl IntoResponse {
    let featured_posts: Vec<&content::blog::BlogPost> = content::blog::BLOG_POSTS.iter().filter_map(|&p| if p.featured { Some(p) } else { None }).collect();

    let template = HomeTemplate { projects, selected_index: 0, technologies: content::tech::TECHNOLOGIES.to_array(), featured_posts };
    HtmlTemplate(template)
}

async fn project(axum::extract::Path(index): axum::extract::Path<String>, axum::extract::State(projects): axum::extract::State<content::project::Projects>) -> impl IntoResponse {
    let template = content::project::ProjectsTemplate { selected_index: index.parse::<usize>().unwrap().into(), projects };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate {
    projects: content::project::Projects,
    selected_index: usize,
    technologies: [&'static content::tech::Technology; 11],
    featured_posts: Vec<&'static content::blog::BlogPost>
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
