use askama_axum::Template;
use axum::{
    extract::Path, extract::State, http::StatusCode, response::{Html, IntoResponse, Response}, routing::get, Router
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;
mod content;

type ProjectsState = Arc<&'static[&'static content::project::Project]>;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_templates=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    
    let projects: ProjectsState = Arc::new(content::project::PROJECTS);
    // build our application with some routes
    let app = Router::new()
        .route("/", get(home))
        .route("/hero", get(hero))
        .route("/project/:selected_index", get(project))
        .route("/projects", get(default_projects_page))
        .route("/projects/:project_name", get(dynamic_projects_page))
        .route("/blog", get(blog_page))
        .route("/featured_posts", get(featured_blog_posts))
        .route("/posts", get(blog))
        .route("/posts/:slug", get(blog_post))
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .with_state(projects);

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

async fn blog_page() -> impl IntoResponse {
    let featured_posts: Vec<&content::blog::BlogPost> = content::blog::BLOG_POSTS.iter().filter_map(|&p| if p.featured { Some(p) } else { None }).collect();
    let template = content::blog::BlogPageTemplate { featured_posts };
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

async fn home(State(projects): State<ProjectsState>) -> impl IntoResponse {
    let featured_posts: Vec<&content::blog::BlogPost> = content::blog::BLOG_POSTS.iter().filter_map(|&p| if p.featured { Some(p) } else { None }).collect();

    let template = HomeTemplate { projects, selected_index: 0, technologies: content::tech::TECHNOLOGIES.to_array(), featured_posts };
    HtmlTemplate(template)
}

async fn hero(State(projects): State<ProjectsState>) -> impl IntoResponse {
    let template = HeroTemplate { projects, technologies: content::tech::TECHNOLOGIES.to_array() };
    HtmlTemplate(template)
}

async fn project(Path(index): Path<usize>, State(projects): State<ProjectsState>) -> impl IntoResponse {
    let template = content::project::ProjectTemplate { selected_index: index, projects };
    HtmlTemplate(template)
}

async fn default_projects_page(State(projects): State<ProjectsState>) -> impl IntoResponse {
    let template = content::project::ProjectsTemplate { selected_index: 0, projects };
    HtmlTemplate(template)
}

async fn dynamic_projects_page( Path(project_name): Path<String>, State(projects): State<ProjectsState>) -> impl IntoResponse {
    let selected_index = projects.iter().position(|&r| r.name == project_name).unwrap();
    let template = content::project::ProjectsTemplate { selected_index: selected_index, projects };
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

#[derive(Template)]
#[template(path = "hero.html")]
struct HeroTemplate {
    projects: content::project::Projects,
    technologies: [&'static content::tech::Technology; 11],
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

