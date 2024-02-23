use yew::prelude::*;
use yew_router::prelude::*;

mod company_login;
mod user_page;

use company_login::CompanyLogin;
use user_page::UserPage;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/company-login")]
    CompanyLogin,
    #[at("/user-page")]
    UserPage,
}

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <div class="home-container">
                <header class="home-header">
                    <h1>{"Blockchain Identity Verification"}</h1>
                    <p>{"Securely verify identities with blockchain technology."}</p>
                </header>
                <section class="home-intro">
                    <p>{"Our platform leverages blockchain to provide a secure and trustworthy environment for identity verification. Whether you're an individual or a company, we ensure your digital interactions are verified and safe."}</p>
                </section>
                <nav class="home-nav">
                    <ul>
                        <li><a class="nav-link" href="/company-login">{"Company Login"}</a></li>
                        <li><a class="nav-link" href="/user-page">{"User Verification"}</a></li>
                    </ul>
                </nav>
            </div>
        },
        Route::CompanyLogin => html! { <CompanyLogin /> },
        Route::UserPage => html! { <UserPage /> },
    }
}

fn main() {
    yew::start_app::<Model>();
}