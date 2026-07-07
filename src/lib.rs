use std::cell::RefCell;
use std::rc::Rc;

use font8x8::{BASIC_FONTS, UnicodeFonts};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, KeyboardEvent, WheelEvent};

const MARGIN: i32 = 40;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window`")
}

// ---------- Resume content model ----------

#[derive(Clone, Copy)]
enum S {
    Name,   // big header
    Sub,    // subtitle / tagline
    Sec,    // section heading
    Role,   // job / school title
    Meta,   // dates, location, contact
    Body,   // paragraph text
    Bullet, // indented bullet line
}

/// scale, color, space-before, indent
fn style_props(s: S) -> (i32, [u8; 4], i32, i32) {
    match s {
        S::Name => (4, [245, 245, 250, 255], 24, 0),
        S::Sub => (2, [120, 200, 255, 255], 4, 0),
        S::Sec => (3, [250, 210, 90, 255], 42, 0),
        S::Role => (2, [245, 245, 250, 255], 24, 0),
        S::Meta => (2, [150, 150, 165, 255], 4, 0),
        S::Body => (2, [205, 208, 215, 255], 10, 0),
        S::Bullet => (2, [190, 196, 208, 255], 4, 18),
    }
}

const CONTENT: &[(S, &str)] = &[
    (S::Name, "FRANK LAN"),
    (S::Sub, "Data and Software Engineer"),
    (S::Meta, "Greater Melbourne Area, Australia"),
    (S::Meta, "franklan118@gmail.com  |  0435 753 413"),
    (S::Meta, "linkedin.com/in/tlan16"),
    (S::Sec, "SUMMARY"),
    (
        S::Body,
        "With 8 years of professional experience, Frank is a versatile software and data engineer adept at creating impactful solutions for both government and private enterprise. His expertise spans high-performance web platforms and ERP systems (Python, TypeScript) to core data platforms and real-time data pipelines (Python, SQL, Kafka, BigQuery, AWS). An effective problem-solver, communicator, and leader, he combines strong project management with a sharp acumen for emerging technologies.",
    ),
    (S::Sec, "TOP SKILLS"),
    (S::Bullet, "- Web Development"),
    (S::Bullet, "- PHP"),
    (S::Bullet, "- JavaScript"),
    (S::Sec, "CERTIFICATIONS"),
    (S::Bullet, "- HackerRank JavaScript (Intermediate)"),
    (S::Bullet, "- HackerRank Problem Solving"),
    (S::Bullet, "- HackerRank Node.js (Intermediate)"),
    (S::Sec, "LANGUAGES"),
    (S::Bullet, "- English (Native or Bilingual)"),
    (S::Sec, "EXPERIENCE"),
    (S::Role, "REA Group - Senior Data Engineer"),
    (S::Meta, "Jun 2024 - Present  |  Melbourne, Australia"),
    (
        S::Body,
        "Contributed to a core property data platform tracking all residential and commercial properties across Australia, powering internal operations and external data products.",
    ),
    (
        S::Bullet,
        "- Built and optimized data pipelines with Python, SQL, and DataFrames, integrating Kafka for real-time streams and BigQuery for analytics.",
    ),
    (
        S::Bullet,
        "- Designed data access interfaces: REST APIs, Kafka topics, and a centralized BigQuery warehouse for BI and reporting.",
    ),
    (
        S::Bullet,
        "- Used AWS (ECS, Lambda, S3) for storage, processing, and deployment.",
    ),
    (S::Role, "REA Group - Senior Software Engineer"),
    (S::Meta, "Jan 2022 - Jun 2024  |  Melbourne, Australia"),
    (
        S::Body,
        "Led full-stack development of property.com.au (Australia's No.3 property research site) from inception to a high-traffic platform.",
    ),
    (
        S::Bullet,
        "- Balanced SEO with rich content using SSR React, variable rendering, and hydration.",
    ),
    (
        S::Bullet,
        "- Owned the user account system: authentication and data privilege isolation.",
    ),
    (
        S::Bullet,
        "- Integrated aerial imagery with external vendors.",
    ),
    (
        S::Bullet,
        "- Used Snowflake and Optimizely for analytics and A/B testing; integrated HubSpot CRM.",
    ),
    (S::Role, "AARNet - Senior Software Engineer"),
    (S::Meta, "Sep 2021 - Jan 2022  |  Melbourne, Australia"),
    (
        S::Body,
        "Primary developer for a new Scalable Data Storage (SDS) backend managing and auditing access to sensitive data, complementing Australia's largest ownCloud deployment.",
    ),
    (
        S::Bullet,
        "- Built a secure REST API with NestJS (TypeScript) and JWT auth.",
    ),
    (
        S::Bullet,
        "- Backend with MariaDB, integrated with the ownCloud SDK.",
    ),
    (S::Bullet, "- Achieved 90%+ test coverage with Jest."),
    (
        S::Bullet,
        "- AWS Kubernetes, RDS, Cognito; Vue.js front-end with UI Kit.",
    ),
    (S::Role, "MSTS - Senior Software Engineer"),
    (S::Meta, "Oct 2019 - Sep 2021  |  Melbourne, Australia"),
    (
        S::Body,
        "Worked on a global B2B layby payment platform, a microservices-based part of their credit-as-a-service suite (~10-member agile team).",
    ),
    (
        S::Bullet,
        "- Improved the API testing pipeline: auto-asserted request/response conformity to OpenAPI for REST and GraphQL (Jest/Cypress).",
    ),
    (
        S::Bullet,
        "- Built backend in TypeScript (NestJS) with dual GraphQL/REST APIs, JWT/API-key auth.",
    ),
    (
        S::Bullet,
        "- PostgreSQL with custom query builders and raw SQL, avoiding ORM abstractions.",
    ),
    (
        S::Bullet,
        "- Middleware and interceptors for auth, logging, and auditing; 90%+ coverage.",
    ),
    (
        S::Bullet,
        "- Vue.js/Quasar frontend; Salesforce Service Cloud integration.",
    ),
    (S::Role, "Balance Internet - Full Stack Developer"),
    (S::Meta, "Aug 2018 - Sep 2019  |  Melbourne, Australia"),
    (
        S::Body,
        "Architected a multi-tenant integration framework syncing order, product, and customer data to ERP systems (Magento, Apparel21, Pronto, BorderFree) via API and FTP.",
    ),
    (
        S::Bullet,
        "- Stack: MongoDB, RabbitMQ, Elasticsearch/Kibana, Kubernetes, Docker, Jenkins, Serverless, PHP (Symfony), Node (Express).",
    ),
    (
        S::Bullet,
        "- Built a REST API via AWS API Gateway + Cognito, passing messages to an RPC queue.",
    ),
    (
        S::Bullet,
        "- 20+ endpoints supporting JSON/XML CRUD via dependency injection.",
    ),
    (
        S::Bullet,
        "- Live anomaly monitoring with elastalert; Kubernetes-based job scheduler.",
    ),
    (S::Role, "eXce Pty Ltd - JavaScript Developer"),
    (S::Meta, "Apr 2018 - Jun 2018  |  Melbourne, Australia"),
    (
        S::Body,
        "Tech lead for a blockchain mobile app to create, transfer, and manage virtual currencies and wallets.",
    ),
    (
        S::Bullet,
        "- React Native (single iOS/Android codebase); Node.js REST API; PostgreSQL.",
    ),
    (
        S::Bullet,
        "- Firebase serverless API; released the first MVP in 40 days.",
    ),
    (S::Role, "nbn Australia - PHP Developer"),
    (S::Meta, "Jan 2017 - Oct 2017  |  Melbourne, Australia"),
    (
        S::Body,
        "Automated deployment of fibre-to-the-curb (FTTC) technology for administrators and field engineers.",
    ),
    (
        S::Bullet,
        "- Zend 2 REST API in PHP; Oracle DB (hundreds of tables, TBs of data); React frontend.",
    ),
    (
        S::Bullet,
        "- Integrations via REST and SOAP; participated in overnight CI/CD deployments.",
    ),
    (S::Role, "Vortilla Holdings - PHP Developer / DevOps"),
    (S::Meta, "Jun 2016 - Dec 2016  |  Docklands, Australia"),
    (
        S::Body,
        "Built a franchise procedure and document management platform.",
    ),
    (
        S::Bullet,
        "- Laravel 5.3 REST API, MySQL, PHPUnit; deployed on AWS (EC2, RDS, S3, Route 53).",
    ),
    (
        S::Bullet,
        "- Designed the DB, migrated legacy data, automated testing and deployment.",
    ),
    (S::Role, "Sysbox Pty Ltd - Full-stack Developer"),
    (S::Meta, "Jun 2014 - Jun 2016  |  Melbourne, Australia"),
    (
        S::Body,
        "Retail startup. Built magento-b2b (10k+ products) and properta, a rental property management platform.",
    ),
    (
        S::Bullet,
        "- PRADO (PHP) API, Prototype JS/jQuery frontend on CentOS.",
    ),
    (
        S::Bullet,
        "- Laravel + AngularJS on AWS; deep Magento 1 integration.",
    ),
    (S::Role, "BPCTECH Limited - Web Developer"),
    (S::Meta, "Nov 2012 - Sep 2015  |  Melbourne, Australia"),
    (
        S::Bullet,
        "- Maintained a Magento eCommerce platform and built a custom ERP plus middleware.",
    ),
    (S::Role, "Enable Development - Full-stack Developer"),
    (S::Meta, "Dec 2014 - Mar 2015  |  West Melbourne, Australia"),
    (
        S::Bullet,
        "- Web platform to help preserve endangered sign languages.",
    ),
    (S::Role, "Larissa Beauty - Market Researcher"),
    (S::Meta, "Jun 2011 - Aug 2011  |  Melbourne, Australia"),
    (
        S::Bullet,
        "- Collected and analyzed competitors' marketing strategy.",
    ),
    (S::Sec, "EDUCATION"),
    (S::Role, "Monash University"),
    (
        S::Body,
        "Honours Bachelor of Engineering (BE), Electrical and Computer Systems Engineering",
    ),
    (S::Role, "Camberwell High School"),
    (S::Body, "VCE"),
    (S::Meta, ""),
    (S::Meta, "-- scroll / arrow keys to navigate --"),
];

// ---------- Framebuffer ----------

struct FrameBuffer {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl FrameBuffer {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height * 4) as usize],
        }
    }
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.pixels.resize((width * height * 4) as usize, 0);
    }
    #[inline]
    fn set_pixel(&mut self, x: u32, y: u32, c: [u8; 4]) {
        if x >= self.width || y >= self.height {
            return;
        }
        let i = ((y * self.width + x) * 4) as usize;
        self.pixels[i..i + 4].copy_from_slice(&c);
    }
}

/// A laid-out line, positioned in document space (before scrolling).
struct Line {
    text: String,
    x: i32,
    y: i32,
    scale: i32,
    color: [u8; 4],
}

struct App {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    fb: FrameBuffer,
    layout: Vec<Line>,
    content_height: i32,
    scroll_y: i32,
}

/// Greedy word-wrap to a maximum number of characters per line (ASCII content).
fn wrap(text: &str, max_chars: usize) -> Vec<String> {
    let max = max_chars.max(1);
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        if word.len() > max {
            if !cur.is_empty() {
                lines.push(std::mem::take(&mut cur));
            }
            let mut w = word;
            while w.len() > max {
                lines.push(w[..max].to_string());
                w = &w[max..];
            }
            cur = w.to_string();
        } else if cur.is_empty() {
            cur = word.to_string();
        } else if cur.len() + 1 + word.len() <= max {
            cur.push(' ');
            cur.push_str(word);
        } else {
            lines.push(std::mem::take(&mut cur));
            cur = word.to_string();
        }
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    if lines.is_empty() {
        lines.push(String::new()); // preserve blank spacer lines
    }
    lines
}

impl App {
    fn resize_to_window(&mut self) {
        let w = window();
        let dpr = w.device_pixel_ratio();                    // e.g. 2.0
        let css_w = w.inner_width().unwrap().as_f64().unwrap();
        let css_h = w.inner_height().unwrap().as_f64().unwrap();
        let px_w = (css_w * dpr) as u32;
        let px_h = (css_h * dpr) as u32;

        self.canvas.set_width(px_w);
        self.canvas.set_height(px_h);
        // CSS size stays in logical pixels so layout is correct
        let style = self.canvas.style();
        style.set_property("width",  &format!("{css_w}px")).unwrap();
        style.set_property("height", &format!("{css_h}px")).unwrap();

        self.fb.resize(px_w, px_h);
    }

    // ----- pixel primitives -----

    fn clear(&mut self, color: [u8; 4]) {
        for p in self.fb.pixels.chunks_exact_mut(4) {
            p.copy_from_slice(&color);
        }
    }

    fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: [u8; 4]) {
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx;
                let py = y + dy;
                if px >= 0 && py >= 0 {
                    self.fb.set_pixel(px as u32, py as u32, color);
                }
            }
        }
    }

    fn draw_char(&mut self, ch: char, x: i32, y: i32, scale: i32, color: [u8; 4]) {
        if let Some(glyph) = BASIC_FONTS.get(ch) {
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..8 {
                    if bits & (1 << col) != 0 {
                        self.fill_rect(
                            x + col as i32 * scale,
                            y + row as i32 * scale,
                            scale,
                            scale,
                            color,
                        );
                    }
                }
            }
        }
    }

    fn draw_text(&mut self, text: &str, x: i32, y: i32, scale: i32, color: [u8; 4]) {
        let mut cx = x;
        for ch in text.chars() {
            self.draw_char(ch, cx, y, scale, color);
            cx += 9 * scale;
        }
    }

    // ----- layout -----

    /// Recompute wrapped line positions. Depends on canvas width, so it runs
    /// on startup and whenever the window resizes.
    fn relayout(&mut self) {
        self.layout.clear();
        let avail = (self.fb.width as i32 - 2 * MARGIN).max(80);
        let mut y = 30;
        for &(style, text) in CONTENT.iter() {
            let (scale, color, space, indent) = style_props(style);
            y += space;
            let x = MARGIN + indent;
            let max_chars = ((avail - indent) / (9 * scale)).max(1) as usize;
            let line_h = 8 * scale + 6;
            for piece in wrap(text, max_chars) {
                self.layout.push(Line {
                    text: piece,
                    x,
                    y,
                    scale,
                    color,
                });
                y += line_h;
            }
        }
        self.content_height = y + 40;
        self.clamp_scroll();
    }

    fn clamp_scroll(&mut self) {
        let max = (self.content_height - self.fb.height as i32).max(0);
        self.scroll_y = self.scroll_y.clamp(0, max);
    }

    fn render(&mut self) {
        self.clear([20, 22, 28, 255]);
        let vh = self.fb.height as i32;
        let sy = self.scroll_y;

        // Move layout ou[118;6:3ut so we can borrow &mut self for drawing, then restore.
        let layout = std::mem::take(&mut self.layout);
        for l in &layout {
            let screen_y = l.y - sy;
            if screen_y + 8 * l.scale < 0 || screen_y > vh {
                continue; // off-screen; skip
            }
            self.draw_text(&l.text, l.x, screen_y, l.scale, l.color);
        }
        self.layout = layout;

        // Scrollbar
        if self.content_height > vh {
            let tx = self.fb.width as i32 - 10;
            self.fill_rect(tx, 0, 6, vh, [40, 42, 50, 255]);
            let thumb_h = (((vh as f32 / self.content_height as f32) * vh as f32) as i32).max(30);
            let max_scroll = (self.content_height - vh).max(1);
            let thumb_y = ((sy as f32 / max_scroll as f32) * (vh - thumb_h) as f32) as i32;
            self.fill_rect(tx, thumb_y, 6, thumb_h, [120, 130, 150, 255]);
        }

        // Blit
        let (w, h) = (self.fb.width, self.fb.height);
        let image =
            ImageData::new_with_u8_clamped_array_and_sh(Clamped(&self.fb.pixels), w, h).unwrap();
        self.ctx.put_image_data(&image, 0.0, 0.0).unwrap();
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = window().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let app = Rc::new(RefCell::new(App {
        canvas,
        ctx,
        fb: FrameBuffer::new(1, 1),
        layout: Vec::new(),
        content_height: 0,
        scroll_y: 0,
    }));
    {
        let mut a = app.borrow_mut();
        a.resize_to_window();
        a.relayout();
        a.render();
    }

    // --- Resize: re-layout for the new width and repaint ---
    {
        let app = app.clone();
        let cb = Closure::<dyn FnMut()>::new(move || {
            let mut a = app.borrow_mut();
            a.resize_to_window();
            a.relayout();
            a.render();
        });
        window().add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // --- Mouse wheel / trackpad: vertical scroll ---
    {
        let app_wheel = app.clone();
        let cb = Closure::<dyn FnMut(WheelEvent)>::new(move |e: WheelEvent| {
            e.prevent_default();
            let mut a = app_wheel.borrow_mut();
            let mut d = e.delta_y();
            if e.delta_mode() == 1 {
                d *= 16.0; // line-based deltas -> pixels
            }
            a.scroll_y += d as i32;
            a.clamp_scroll();
            a.render();
        });
        app.borrow()
            .canvas
            .add_event_listener_with_callback("wheel", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // --- Keyboard: arrows / PageUp-Down / Space / Home / End (still vertical scroll) ---
    {
        let app = app.clone();
        let cb = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            let mut a = app.borrow_mut();
            let page = (a.fb.height as i32 - 40).max(40);
            let handled = match e.key().as_str() {
                "ArrowDown" => {
                    a.scroll_y += 48;
                    true
                }
                "ArrowUp" => {
                    a.scroll_y -= 48;
                    true
                }
                "PageDown" | " " => {
                    a.scroll_y += page;
                    true
                }
                "PageUp" => {
                    a.scroll_y -= page;
                    true
                }
                "Home" => {
                    a.scroll_y = 0;
                    true
                }
                "End" => {
                    a.scroll_y = a.content_height;
                    true
                }
                _ => false,
            };
            if handled {
                e.prevent_default();
                a.clamp_scroll();
                a.render();
            }
        });
        window().add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    Ok(())
}
