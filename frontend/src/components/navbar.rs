use yew::{function_component, html};

#[function_component(Navbar)]
pub fn app() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-light bg-light">
        <div class="collapse navbar-collapse" id="navbarNav">
          <ul class="navbar-nav">
            <li class="nav-item">
              <a class="nav-link" href="/index">{"Home"}</a>
            </li>
            <li class="nav-item">
              <a class="nav-link" href="/private/create">{"Add private board"}</a>
            </li>
            <li class="nav-item">
              <a class="nav-link" href="/team/create">{"Create team"}</a>
            </li>
            <li class="nav-item">
              <a class="nav-link" href="/team_board/create">{"Add team board"}</a>
            </li>
          </ul>
        </div>
      </nav>
    }
}
