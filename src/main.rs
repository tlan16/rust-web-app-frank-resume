use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

const MAX_WIDTH: f32 = 760.0;
const PAD_X: f32 = 24.0;
const LINE: f32 = 24.0;

#[derive(Clone, Copy)]
enum S {
    Name,
    Sub,
    Sec,
    Role,
    Meta,
    Body,
    Bullet,
}

/// font_size, color, margin_top, margin_left
fn style_props(s: S) -> (f32, Color, f32, f32) {
    match s {
        S::Name => (40.0, Color::srgb_u8(245, 245, 250), 24.0, 0.0),
        S::Sub => (22.0, Color::srgb_u8(120, 200, 255), 4.0, 0.0),
        S::Sec => (28.0, Color::srgb_u8(250, 210, 90), 42.0, 0.0),
        S::Role => (22.0, Color::srgb_u8(245, 245, 250), 24.0, 0.0),
        S::Meta => (15.0, Color::srgb_u8(150, 150, 165), 4.0, 0.0),
        S::Body => (17.0, Color::srgb_u8(205, 208, 215), 10.0, 0.0),
        S::Bullet => (17.0, Color::srgb_u8(190, 196, 208), 4.0, 18.0),
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
    (S::Bullet, "- Integrated aerial imagery with external vendors."),
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
    (S::Bullet, "- Backend with MariaDB, integrated with the ownCloud SDK."),
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
    (S::Body, "Built a franchise procedure and document management platform."),
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
    (S::Bullet, "- Laravel + AngularJS on AWS; deep Magento 1 integration."),
    (S::Role, "BPCTECH Limited - Web Developer"),
    (S::Meta, "Nov 2012 - Sep 2015  |  Melbourne, Australia"),
    (
        S::Bullet,
        "- Maintained a Magento eCommerce platform and built a custom ERP plus middleware.",
    ),
    (S::Role, "Enable Development - Full-stack Developer"),
    (S::Meta, "Dec 2014 - Mar 2015  |  West Melbourne, Australia"),
    (S::Bullet, "- Web platform to help preserve endangered sign languages."),
    (S::Role, "Larissa Beauty - Market Researcher"),
    (S::Meta, "Jun 2011 - Aug 2011  |  Melbourne, Australia"),
    (S::Bullet, "- Collected and analyzed competitors' marketing strategy."),
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


#[derive(Component)]
struct ScrollBox;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Reuse your existing <canvas id="canvas">
                canvas: Some("#canvas".into()),
                fit_canvas_to_parent: true,
                // Stop the browser from scrolling / eating arrow keys
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (mouse_scroll, keyboard_scroll))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            // Full-screen root, horizontally centers the resume column
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb_u8(20, 22, 28)),
        ))
        .with_children(|root| {
            root.spawn((
                ScrollBox,
                Node {
                    width: Val::Px(MAX_WIDTH),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    overflow: Overflow::scroll_y(), // <-- the built-in scroll container
                    padding: UiRect::axes(Val::Px(PAD_X), Val::Px(30.0)),
                    ..default()
                },
            ))
                .with_children(|list| {
                    for &(style, text) in CONTENT.iter() {
                        let (font_size, color, mt, ml) = style_props(style);
                        list.spawn((
                            Text::new(text),
                            TextFont {
                                font_size,
                                ..default() // uses the embedded default_font (a real TTF)
                            },
                            TextColor(color),
                            Node {
                                margin: UiRect {
                                    top: Val::Px(mt),
                                    left: Val::Px(ml),
                                    ..default()
                                },
                                max_width: Val::Px(MAX_WIDTH - 2.0 * PAD_X),
                                ..default()
                            },
                        ));
                    }
                });
        });
}

fn mouse_scroll(mut wheel: EventReader<MouseWheel>, mut q: Query<&mut ScrollPosition, With<ScrollBox>>) {
    let mut dy = 0.0;
    for ev in wheel.read() {
        dy += match ev.unit {
            MouseScrollUnit::Line => ev.y * LINE,
            MouseScrollUnit::Pixel => ev.y,
        };
    }
    if dy != 0.0 {
        for mut sp in &mut q {
            sp.offset_y = (sp.offset_y - dy).max(0.0);
        }
    }
}

fn keyboard_scroll(keys: Res<ButtonInput<KeyCode>>, mut q: Query<&mut ScrollPosition, With<ScrollBox>>) {
    let mut delta = 0.0;
    let mut jump: Option<f32> = None;

    if keys.pressed(KeyCode::ArrowDown) {
        delta += 12.0;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        delta -= 12.0;
    }
    if keys.just_pressed(KeyCode::PageDown) || keys.just_pressed(KeyCode::Space) {
        delta += 500.0;
    }
    if keys.just_pressed(KeyCode::PageUp) {
        delta -= 500.0;
    }
    if keys.just_pressed(KeyCode::Home) {
        jump = Some(0.0);
    }
    if keys.just_pressed(KeyCode::End) {
        jump = Some(1.0e6); // Bevy clamps to the content bottom
    }

    if delta != 0.0 || jump.is_some() {
        for mut sp in &mut q {
            sp.offset_y = jump.unwrap_or((sp.offset_y + delta).max(0.0));
        }
    }
}
