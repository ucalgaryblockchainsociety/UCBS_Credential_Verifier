use yew::prelude::*;

pub struct CompanyLogin;

impl Component for CompanyLogin {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="login-container">
                <h2 class="login-title">{"Company Login"}</h2>
                <form class="login-form">
                    <div class="form-group">
                        <input type="text" placeholder="Company ID" class="form-control" />
                    </div>
                    <div class="form-group">
                        <input type="password" placeholder="Password" class="form-control" />
                    </div>
                    <button class="btn btn-primary">{"Login"}</button>
                </form>
            </div>
        }
    }
}