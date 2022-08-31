use yew::prelude::*;
use yew::ServerRenderer;

#[function_component]
pub fn MenuItemInnerInner(MenuItemProps { name, href }: &MenuItemProps) -> Html {
    html! {
        <strong>
            <a this="item" has="many" attributes="so" that="we" can="test" if="that" is="an" issue="." href={href}>{name}</a>
        </strong>
    }
}

#[function_component]
pub fn MenuItemInner(MenuItemProps { name, href }: &MenuItemProps) -> Html {
    html! {
        <em>
            <MenuItemInnerInner name={name} href={href} />
        </em>
    }
}

#[derive(Properties, PartialEq)]
pub struct MenuItemProps {
    name: AttrValue,
    href: AttrValue,
}

#[function_component]
pub fn MenuItem(MenuItemProps { name, href }: &MenuItemProps) -> Html {
    html! {
        <li this="item" has="many" attributes="so" that="we" can="test" if="that" is="an" issue=".">
            <span this="item" has="many" attributes="so" that="we" can="test" if="that" is="an" issue=".">
                <div this="item" has="many" attributes="so" that="we" can="test" if="that" is="an" issue=".">
                    <form this="item" has="many" attributes="so" that="we" can="test" if="that" is="an" issue=".">
                        <span this="item" has="many" attributes="so" that="we" can="test" if="that" is="an" issue=".">
                            <MenuItemInner name={name} href={href} />
                        </span>
                    </form>
                </div>
            </span>
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct MenuProps {
    items: Vec<AttrValue>,
}

#[function_component]
pub fn Menu(MenuProps { items }: &MenuProps) -> Html {
    html! {
        <ul>
            {for items.iter().map(|name| {
                html! {
                    <MenuItem name={name} href={name} />
                }
            })}
        </ul>
    }
}

#[derive(Properties, PartialEq)]
pub struct StackedProps {
    pub children: Children,
}

#[function_component]
pub fn Stacked(StackedProps { children }: &StackedProps) -> Html {
    html! {
        <div class="grid">
            {for children.iter()}
        </div>
    }
}

#[function_component]
pub fn Header() -> Html {
    html! {
        <header>
            <h1>{"Profiling test"}</h1>

            <p>{"
                Rust is a multi-paradigm, general-purpose programming language.
                Rust emphasizes performance, type safety, and concurrency.
                Rust enforces memory safety—that is, that all references point to valid memory—without requiring the use of a garbage collector or reference counting present in other memory-safe languages.
                To simultaneously enforce memory safety and prevent concurrent data races, Rust's borrow checker tracks the object lifetime and variable scope of all references in a program during compilation.
                Rust is popular for systems programming but also offers high-level features including functional programming constructs.
            "}</p>

            <p>{"
                Rust is a multi-paradigm, general-purpose programming language.
                Rust emphasizes performance, type safety, and concurrency.
                Rust enforces memory safety—that is, that all references point to valid memory—without requiring the use of a garbage collector or reference counting present in other memory-safe languages.
                To simultaneously enforce memory safety and prevent concurrent data races, Rust's borrow checker tracks the object lifetime and variable scope of all references in a program during compilation.
                Rust is popular for systems programming but also offers high-level features including functional programming constructs.
            "}</p>
        </header>
    }
}

#[function_component]
pub fn Footer() -> Html {
    html! {
        <footer>
            {for (1..20).into_iter().map(|_| html! {
                <p>{"
                    To support our vibrant community, we provide the essential infrastructure and organizational framework for the development of multilingual wiki Projects and their editions (as explained here) and other endeavors which serve this mission.

                    We strive to make and keep educational and informational content from the Projects available on the internet free of charge, in perpetuity.





            "}</p>})}
        </footer>
    }
}

#[derive(Properties, PartialEq)]
pub struct WrapperProps {
    menu_items: Vec<AttrValue>,
}

#[function_component]
pub fn Wrapper(WrapperProps { menu_items }: &WrapperProps) -> Html {
    html! {
        <Stacked>
            <Header />
            <Menu items={menu_items.clone()}/>
            <Footer />
        </Stacked>
    }
}

#[function_component]
pub fn App() -> Html {
    let menu_items = ["Home", "About", "Projects", "Login", "Register", "Contact"]
        .repeat(25)
        .into_iter()
        .map(AttrValue::from)
        .collect::<Vec<_>>();

    html! {
        <Stacked>
            <Wrapper menu_items={menu_items.clone()} />
            <Wrapper menu_items={menu_items.clone()} />
            <Wrapper menu_items={menu_items.clone()} />
            <Wrapper menu_items={menu_items.clone()} />
            <Wrapper menu_items={menu_items} />
        </Stacked>
    }
}

#[tokio::main]
async fn main() {
    let renderer = ServerRenderer::<App>::new();

    println!("{}", renderer.render().await);
}
