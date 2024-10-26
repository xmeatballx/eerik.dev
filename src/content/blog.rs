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
        title: "My NixOS Laptop Setup",
        excerpt: "I like customizing things. So when the hard drive on my laptop died, wiping my Windows install that I had been running for 3 years...",
        slug: "nixos-laptop",
        markdown_path: "nixos.md",
        published_date: "10/17/24",
        featured_image: "/assets/images/nixos.png",
        featured: false
    },
    &BlogPost{ 
        title: "Setting up a Homelab",
        excerpt: "After a month of Reddit suggesting posts from a community called Homelab based on my other interests, I gave in to curiosity...",
        slug: "homelab",
        markdown_path: "homelab.md",
        published_date: "7/20/24",
        featured_image: "/assets/images/homelab.png",
        featured: true
    },
    &BlogPost{ 
        title: "Custom NodeJS Notion integration",
        excerpt: "Notion is a powerful knowledge management system and productivity tool...",
        slug: "notion",
        markdown_path: "notion_integration/notion_integration.md",
        published_date: "3/14/24",
        featured_image: "/assets/images/notion_integration.png",
        featured: true
    },
    &BlogPost{ 
        title: "Generating color palettes with musical ratios",
        excerpt: "As a developer with a background in music composition, I had an idea for a web app called ColorChords...",
        slug: "colorchords",
        markdown_path: "colorchords.md",
        published_date: "11/11/23",
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
#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogPageTemplate { 
    pub featured_posts: Vec<&'static BlogPost>,
    pub posts: &'static[&'static BlogPost]
}
