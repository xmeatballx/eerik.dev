use askama_axum::Template;

pub struct BlogPost {
    pub title: &'static str,
    pub excerpt: &'static str,
    pub slug: &'static str,
    pub markdown_path: &'static str,
    pub published_date: &'static str,
    pub featured_image: &'static str,
    pub featured: bool,
}

pub static BLOG_POSTS: &'static[&'static BlogPost] = &[
    &BlogPost{ 
        title: "Setting up a Homelab",
        excerpt: "I recently embarked on a journey to create my own homelab server for media management, self-hosting, and virtual machine experiments...",
        slug: "homelab",
        markdown_path: "homelab.md",
        published_date: "11/11/11",
        featured_image: "/assets/images/homelab.png",
        featured: true
    },
    &BlogPost{ 
        title: "Custom NodeJS notion integration",
        excerpt: "Notion is a powerful knowledge management system and productivity tool. I'll show you how to encorporate your Notion data in a basic web app...",
        slug: "notion",
        markdown_path: "notion_integration/notion_integration.md",
        published_date: "11/11/11",
        featured_image: "/assets/images/notion_integration.png",
        featured: true
    },
    &BlogPost{ 
        title: "Generating color palettes with musical ratios",
        excerpt: "As a developer with a background in music composition, I had an idea for a web app called ColorChords, which translated musical harmonic ratios into color space...",
        slug: "colorchords",
        markdown_path: "colorchords.md",
        published_date: "11/11/11",
        featured_image: "/assets/images/colorchords.png",
        featured: true
    },
];

#[derive(Template)]
#[template(path = "blog_post.html")]
pub struct BlogPostTemplate { 
    pub post: &'static BlogPost,
    pub html: String,
}


#[derive(Template)]
#[template(path = "blog_all_posts.html")]
pub struct BlogAllPostsTemplate { 
    pub posts: &'static[&'static BlogPost],
}

#[derive(Template)]
#[template(path = "featured_blogs.html")]
pub struct FeaturedBlogsTemplate { 
    pub featured_posts: Vec<&'static BlogPost>
}
