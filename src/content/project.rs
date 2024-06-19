use std::fmt;
use askama_axum::Template;
use crate::content::tech;

pub enum MediaFormat {
    Image,
    Video
}

impl MediaFormat {
    pub fn is_video(&self) -> bool {
        matches!(self, MediaFormat::Video)
    }

    pub fn is_image(&self) -> bool {
        matches!(self, MediaFormat::Image)
    }
}

impl fmt::Display for MediaFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})",self)
    }
}

#[derive(Clone)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub media_url: &'static str,
    pub media_format: &'static MediaFormat,
    pub tech: &'static[&'static tech::Technology]
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(name: {},description: {})", self.name, self.description)
    }
}

pub type Projects = &'static[&'static Project];

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {
    pub projects: Projects,
    pub selected_index: usize
}

pub static PROJECTS: Projects = &[ 
        &Project { 
            name: "mDash", 
            description: "Share estimates, timelines, and deliverables with clients and update/notify them as you make progress. Get assistance from AI on the less exciting work like assigning tasks, writing reports, and estimating budget and timelines.",
            media_url: "./assets/images/mdash_demo.mp4",
            media_format: &MediaFormat::Video,
            tech: &[ 
                tech::TECHNOLOGIES.react, 
                tech::TECHNOLOGIES.nodejs, 
                tech::TECHNOLOGIES.mongodb, 
                tech::TECHNOLOGIES.tailwind,
                tech::TECHNOLOGIES.aws
            ]
        },
        &Project { 
            name: "NotionEmbed", 
            description: "Create your own embeddable widget that cycles through selected items in a Notion database. It can be used to show random excerpts from recent notes, see different motivational quotes today, or even add a fun personalized slideshow to a website.", 
            media_url: "./assets/images/notionembed.png",
            media_format: &MediaFormat::Image,
            tech: &[ 
                tech::TECHNOLOGIES.sveltekit, 
                tech::TECHNOLOGIES.postgres, 
                tech::TECHNOLOGIES.redis, 
            ]
        },
        &Project {
            name: "Looker",
            description: "A simple file browser built with Rust, Tauri, and SolidJS. I wanted something fully customized to my laptop setup with only the necessary tools and no distractions (and also an excuse to experiment with new technologies). ",
            media_url: "./assets/images/looker.png",
            media_format: &MediaFormat::Image,
            tech: &[
                tech::TECHNOLOGIES.rust,
                tech::TECHNOLOGIES.htmx,
                tech::TECHNOLOGIES.tailwind
            ]
        },
        &Project { 
            name: "Colorchords",
            description: "A color palette generator based on musical ratios. I made it as an exercise in building framework-free interactive UI's with vanilla JS, CSS, and HTML.",
            media_url: "./assets/images/colorchords.png",
            media_format: &MediaFormat::Image,
            tech: &[ 
                tech::TECHNOLOGIES.html, 
                tech::TECHNOLOGIES.css, 
                tech::TECHNOLOGIES.javascript, 
            ]
        },
        &Project { 
            name: "Wash Rover", 
            description: "...", 
            media_url: "./assets/images/washrover.png",
            media_format: &MediaFormat::Image,
            tech: &[ 
                tech::TECHNOLOGIES.nextjs, 
                tech::TECHNOLOGIES.sailsjs, 
                tech::TECHNOLOGIES.mysql, 
                tech::TECHNOLOGIES.aws, 
            ]
        },
    ];
