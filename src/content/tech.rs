pub struct Technology {
    pub name: &'static str,
    pub fa_classname: &'static str,
    pub text_classname: &'static str,
    pub featured: bool,
}

pub struct Technologies {
    pub typescript: &'static Technology,
    pub react: &'static Technology,
    pub nodejs: &'static Technology,
    pub mongodb: &'static Technology,
    pub docker: &'static Technology,
    pub aws: &'static Technology,
    pub tailwind: &'static Technology,
    pub nextjs: &'static Technology,
    pub sveltekit: &'static Technology,
    pub postgres: &'static Technology,
    pub redis: &'static Technology,
    pub sailsjs: &'static Technology,
    pub mysql: &'static Technology,
    pub html: &'static Technology,
    pub css: &'static Technology,
    pub javascript: &'static Technology,
    pub rust: &'static Technology,
    pub htmx: &'static Technology,
}

impl Technologies {
    pub fn to_array(&self) -> [&'static Technology; 11] {
        [
            self.typescript,
            self.nodejs,
            self.react,
            self.mongodb,
            self.aws,
            self.docker,
            self.tailwind,
            self.nextjs,
            self.sveltekit,
            self.rust,
            self.htmx
        ]
    }
}

pub const TECHNOLOGIES: Technologies = Technologies {
    react: &Technology {
        name: "React",
        fa_classname: "fa-brands fa-react text-neutral-content",
        text_classname: "text-neutral-content",
        featured: true
    },
    nodejs: &Technology {
        name: "NodeJS",
        fa_classname: "fa-brands fa-node-js text-success",
        featured: true,
        text_classname: "text-success",
    },
    mongodb: &Technology {
        name: "Mongo DB",
        fa_classname: "fa-solid fa-database text-success",
        featured: true,
        text_classname: "text-success",
    },
    docker: &Technology {
        name: "Docker",
        fa_classname: "fa-brands fa-docker text-primary",
        featured: true,
        text_classname: "text-primary",
    },
    typescript: &Technology {
        name: "Typescript",
        fa_classname: "fa-brands fa-js text-primary",
        featured: true,
        text_classname: "text-primary",
    },
    aws: &Technology {
        name: "AWS",
        fa_classname: "fa-brands fa-aws text-warning",
        featured: true,
        text_classname: "text-warning",
    },
    tailwind: &Technology {
        name: "Tailwind",
        fa_classname: "fa-brands fa-css3-alt text-accent",
        featured: true,
        text_classname: "text-accent",
    },
    nextjs: &Technology {
        name: "NextJS",
        fa_classname: "fa-solid fa-n",
        featured: true,
        text_classname: "",
    },
    sveltekit: &Technology {
        name: "Sveltekit",
        fa_classname: "fa-solid fa-s text-accent",
        featured: false,
        text_classname: "",
    },
    postgres: &Technology {
        name: "Postgres",
        fa_classname: "fa-solid fa-database text-accent",
        featured: false,
        text_classname: "",
    },
    redis: &Technology {
        name: "Redis",
        fa_classname: "fa-solid fa-database text-accent",
        featured: false,
        text_classname: "",
    },
    sailsjs: &Technology {
        name: "SailsJS",
        fa_classname: "fa-solid fa-s",
        featured: false,
        text_classname: "",
    },
    mysql: &Technology {
        name: "mySQL",
        fa_classname: "fa-solid fa-database text-warning",
        featured: false,
        text_classname: "",
    },
    html: &Technology {
        name: "HTML",
        fa_classname: "fa-brands fa-html5 text-accent",
        featured: false,
        text_classname: "",
    },
    css: &Technology {
        name: "CSS",
        fa_classname: "fa-brands fa-css3-alt text-warning",
        featured: false,
        text_classname: "",
    },
    javascript: &Technology {
        name: "Javascript",
        fa_classname: "fa-brands fa-js text-warning",
        featured: false,
        text_classname: "",
    },
    rust: &Technology {
        name: "Rust",
        fa_classname: "fa-brands fa-rust",
        featured: false,
        text_classname: ""
    },
    htmx: &Technology {
        name: "HTMX",
        fa_classname: "fa-solid fa-h",
        featured: false,
        text_classname: ""
    }
};

