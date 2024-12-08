pub mod app {
    use leptos::*;
    use leptos_router::*;

    #[component]
    pub fn App() -> impl IntoView {
        view! {

            <Router>
                <Routes>
                    <Route path=Page::Home.path() view=|| view! { <Home/> }/>
                    <Route path=Page::Blog.path() view=|| view! { <Blog/> }/>
                    <Route path=Page::About.path() view=|| view! { <About/> }/>
                    <Route path="/my-first-blog" view=|| view! { <Post/> }/>
                    <Route path="/*any" view=|| view! { <NotFound/> }/>
                </Routes>
            </Router>
        }
    }

    #[component]
    fn PageLayout(children: Children) -> impl IntoView {
        view! {
            <NavBar/>
            <main class="font-body text-gray-600">
                {children()}
            </main>
            <Footer/>
        }
    }

    #[component]
    fn PageContent(children: Children) -> impl IntoView {
        view! {
            <PageLayout>
                <div class="px-10 m-3">
                    {children()}
                </div>
            </PageLayout>
        }
    }
    #[component]
    fn NavBar() -> impl IntoView {
        let (is_hidden, set_is_hidden) = create_signal(0);

        let toggle_class = { move |_ev| set_is_hidden.update(|is_hidden| *is_hidden += 1) };
        view! {
            <nav class="pb-10">
                <div class="bg-gray-100 flex justify-around items-center">
                    <a href=Page::Home.path() class="py-4">
                        <img class="w-28" src="/images/oblivion-logo.png"/>
                    </a>

                    <a
                        href="#"
                        class="cursor-point md:hidden"
                        on:click=toggle_class
                    >
                        <svg class="w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"> <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 9h16.5m-16.5 6.75h16.5" /> </svg>
                    </a>

                    <div
                        class="hidden md:block overflow-y-auto"
                    >
                        <ul class="grid grid-rows-2 sm:grid-cols-2 space-x-2 text-sm text-center font-bold text-gray-700 uppercase tracking-wider">
                            <li class=""><a href=Page::Blog.path() class="hover:border-b-2 border-primary">"Blog"</a></li>
                            <li class="hover:border-b-2 border-primary"><a href=Page::About.path() class="hover:border-b-2 border-primary">"About"</a></li>
                        </ul>
                    </div>
                </div>
                <div
                    class="md:hidden pt-5"
                    class:hidden={ move || is_hidden.get() % 2 == 0 }
                >
                    <ul class="grid grid-rows-2 sm:grid-cols-2 space-y-2 text-sm text-center font-bold text-gray-700 uppercase tracking-wider">
                        <li class=""><a href=Page::Blog.path() class="hover:border-b-2 border-primary">"Blog"</a></li>
                        <li class="hover:border-b-2 border-primary"><a href=Page::About.path() class="hover:border-b-2 border-primary">"About"</a></li>
                    </ul>
                </div>
            </nav>
        }
    }

    #[component]
    fn Home() -> impl IntoView {
        view! {
            <PageContent>
                <div >
                    <Hero/>
                    <TitleListSection/>
                </div>
            </PageContent>
        }
    }

    #[component]
    fn Blog() -> impl IntoView {
        view! {
            <PageContent>
                <div class="container">
                    <div class="pb-3">
                        <h2 class="font-bold text-2xl">"All Posts"</h2>
                    </div>
                    <div>
                        <ul class="list-disc list-outside">
                            <li>
                                <a class="text-lg border-b border-primary" href="#">"My First Blog"</a>
                                <p class="pt-2">"This summarises my first post"</p>
                            </li>
                        </ul>
                    </div>
                </div>
            </PageContent>
        }
    }

    #[component]
    fn Hero() -> impl IntoView {
        view! {
            <section class="container pb-4">
                <div class="pb-6">
                    <h2 class="text-3xl font-bold">"The hub for the refined thoughts of a wondering mind."</h2>
                </div>
                <div>
                    <p class="text-lg font-medium">"Mathematics, reflections, software and more."</p>
                </div>
            </section>
        }
    }

    #[component]
    fn TitleListSection() -> impl IntoView {
        view! {
            <section class="container overflow-y-auto pt-10">
                <div>
                    <div class="pb-6">
                        <h3 class="text-lg font-bold">"Latest Post"</h3>
                        <div>
                            <ul class="list-disc list-inside">
                                <li><a class="border-b border-primary" href="/my-first-blog">"My First Blog"</a></li>
                            </ul>
                        </div>
                    </div>
                    <div>
                        <h3 class="text-lg font-bold">"Top Picks by Author"</h3>
                        <div>
                            <ul class="list-disc list-inside">
                                <li><a class="border-b border-primary" href="/my-first-blog">"My First Blog"</a></li>
                            </ul>
                        </div>
                    </div>
                </div>
            </section>
        }
    }

    #[component]
    fn Post() -> impl IntoView {
        view! {
            <PageContent>
                <div>
                    <h2 class="font-bold text-xl">"My First Blog"</h2>
                    <br/>
                </div>
                <div>
                   <p>"hi"</p>
                </div>
                // {move || {
                //     view! {<div inner_html={a()}></div>}
                // }}
            </PageContent>
        }
    }

    #[component]
    fn About() -> impl IntoView {
        view! {
            <PageContent>
                <div class="container">
                    <div>
                        <h2>"About The Author"</h2>
                    </div>
                    <div>
                        <p>"Hi, my name is Casper"</p>
                        <p>"I am a ghost."</p>
                    </div>
                </div>
            </PageContent>
        }
    }

    #[component]
    fn Footer() -> impl IntoView {
        view! {
            <footer class="container">
                <div class="px-10 pt-5 pb-10 bg-gray-100 mt-20">
                    <div>
                        <p><span class="font-bold">"Oblivion Times "</span> "publication by "<span><a class="border-b border-gray-900" href="#">"Jethro Djan"</a></span> ". All rights reserved."</p>
                    </div>
                </div>
            </footer>
        }
    }

    #[component]
    pub fn NotFound() -> impl IntoView {
        view! {
           <PageContent>
               <div class="container">
                   <div>
                       <h2>"Requested Page Not Found"</h2>
                   </div>
                   <div>
                       <p>"It looks the page you requested does not exist."</p>
                   </div>
               </div>
           </PageContent>

        }
    }

    pub enum Page {
        Home,
        Blog,
        About,
    }

    impl Page {
        pub fn path(&self) -> &'static str {
            match self {
                Self::Home => "/",
                Self::Blog => "/blog",
                Self::About => "/about",
            }
        }
    }
}

pub mod server {
    use pulldown_cmark::Parser;
    use std::fs;
    use std::path::Path;

    pub fn read_markdown_file(markdown_file_path: &str) -> String {
        let path = Path::new(markdown_file_path);
        let display = path.display();

        let contents = match fs::read_to_string(&path) {
            Err(why) => panic!("Couldn't read {}: {}", display, why),
            Ok(contents) => contents,
        };
        contents
    }

    pub fn convert_to_html_string(markdown_content: String) -> String {
        let parser = Parser::new(markdown_content.as_str());
        let mut html_string = String::new();
        pulldown_cmark::html::push_html(&mut html_string, parser);
        html_string
    }

    pub async fn fetch_blogposts(mut current_blogpost: String) -> String {
        let blogpost = read_markdown_file("/users/jethro/Documents/projects/rs-oblivion-times/public/posts/2024-11-12-my-first-blog.md");
        if current_blogpost == blogpost {
            convert_to_html_string(current_blogpost)
        } else {
            current_blogpost = blogpost;
            convert_to_html_string(current_blogpost)
        }
    }

    #[derive(Clone)]
    pub struct Blogpost {
        pub post_header: String,
        pub post_content: Vec<String>,
    }
}
