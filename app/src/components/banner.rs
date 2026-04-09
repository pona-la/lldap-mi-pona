use crate::components::{
    avatar::Avatar,
    logout::LogoutButton,
    router::{AppRoute, Link},
};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{Callback, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_admin: bool,
    pub username: Option<String>,
    pub on_logged_out: Callback<()>,
}

#[function_component(Banner)]
pub fn banner(props: &Props) -> Html {
    html! {
      <header class="p-2 mb-3 border-bottom">
        <div class="container">
          <div class="d-flex flex-wrap align-items-center justify-content-center justify-content-lg-start">
            <a href={yew_router::utils::base_url().unwrap_or("/".to_string())} class="d-flex align-items-center mb-lg-0 me-md-5 text-decoration-none">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 500 500" height="42" width="42"><circle cx="250" cy="250" r="242.643" fill="#fff"></circle><g stroke="#000"><path fill="none" stroke-width="32.28368" d="M185.433 199.337c-13.41-126.154 130.376-118.456 130.376-45.445"></path><path fill="#c3c3c3" stroke-width="29.80032" d="M172.271 193.625H327.73c13.716 0 24.834 11.118 24.834 24.834v155.458c0 13.716-11.118 24.834-24.834 24.834H172.271c-13.713 0-24.834-11.118-24.834-24.834V218.46c0-13.716 11.121-24.834 24.834-24.834Z"></path><path fill="none" stroke-linecap="round" stroke-linejoin="round" stroke-width="32.28368" d="M198.098 285.013c0 65.064 103.804 65.064 103.804 0"></path></g></svg>
            </a>

            <ul class="nav col-12 col-lg-auto me-lg-auto mb-2 justify-content-center mb-md-0">
              {if props.is_admin { html! {
                <>
                  <li>
                    <Link
                      classes="nav-link px-2 h6"
                      to={AppRoute::ListUsers}>
                      <i class="bi-people me-2"></i>
                      {"Users"}
                    </Link>
                  </li>
                  <li>
                    <Link
                      classes="nav-link px-2 h6"
                      to={AppRoute::ListGroups}>
                      <i class="bi-collection me-2"></i>
                      {"Groups"}
                    </Link>
                  </li>
                  <li>
                    <Link
                      classes="nav-link px-2 h6"
                      to={AppRoute::ListUserSchema}>
                      <i class="bi-list-ul me-2"></i>
                      {"User schema"}
                    </Link>
                  </li>
                  <li>
                    <Link
                      classes="nav-link px-2 h6"
                      to={AppRoute::ListGroupSchema}>
                      <i class="bi-list-ul me-2"></i>
                      {"Group schema"}
                    </Link>
                  </li>
                </>
              } } else { html!{} } }
            </ul>
            <UserMenu username={props.username.clone()} on_logged_out={props.on_logged_out.clone()}/>
            <DarkModeToggle />
          </div>
        </div>
      </header>
    }
}

#[derive(Properties, PartialEq)]
struct UserMenuProps {
    pub username: Option<String>,
    pub on_logged_out: Callback<()>,
}

#[function_component(UserMenu)]
fn user_menu(props: &UserMenuProps) -> Html {
    match &props.username {
        Some(username) => html! {
          <div class="dropdown text-end">
            <a href="#"
              class="d-block nav-link text-decoration-none dropdown-toggle"
              id="dropdownUser"
              data-bs-toggle="dropdown"
              aria-expanded="false">
              <Avatar user={username.clone()} />
              <span class="ms-2">
                {username}
              </span>
            </a>
            <ul
              class="dropdown-menu text-small dropdown-menu-lg-end"
              aria-labelledby="dropdownUser1"
              style="">
              <li>
                <Link
                  classes="dropdown-item"
                  to={AppRoute::UserDetails{ user_id: username.to_string() }}>
                  {"View details"}
                </Link>
              </li>
              <li><hr class="dropdown-divider" /></li>
              <li>
                <LogoutButton on_logged_out={props.on_logged_out.clone()} />
              </li>
            </ul>
          </div>
        },
        _ => html! {},
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = darkmode)]
    fn toggleDarkMode(doSave: bool);

    #[wasm_bindgen]
    fn inDarkMode() -> bool;
}

#[function_component(DarkModeToggle)]
fn dark_mode_toggle() -> Html {
    html! {
      <div class="form-check form-switch">
        <input class="form-check-input" onclick={|_| toggleDarkMode(true)} type="checkbox" id="darkModeToggle" checked={inDarkMode()}/>
        <label class="form-check-label" for="darkModeToggle">{"Dark mode"}</label>
      </div>
    }
}
