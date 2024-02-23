use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/login")]
    Login,
    #[at("/signup")]
    SignUp,
}

pub struct UserPage;

impl Component for UserPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="user-page-container">
                <h2 class="user-page-title">{"User Page"}</h2>
                <div class="user-content">
                    <p>{"Welcome! Please login or sign up to continue."}</p>
                    <div class="user-actions">
                        <Link<Route> to={Route::Login} classes="btn btn-primary">{"Login"}</Link<Route>>
                        <Link<Route> to={Route::SignUp} classes="btn btn-secondary">{"Sign Up"}</Link<Route>>
                    </div>
                </div>
            </div>
        }
    }
}