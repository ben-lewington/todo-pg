use crate::Todo;

lfml::template!(pub base(inner: impl lfml::Render) {
    (lfml::DOCTYPE)
    html .js-disabled lang="en" {
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link href="/static/css/index.css" rel="stylesheet";
            link href="/static/favicon.ico" rel="icon" type="image/x-icon";
            script src="/static/js/index.js" type="module" defer {}
            script src="/static/js/vendor/htmx.min.js" defer {}
        }
        body { (inner) }
        script { "document.documentElement.classList.toggle('js-disabled')" }
    }
});

pub enum Hypermedia<T> {
    Document(T),
    Fragment(T),
}

impl<T: lfml::Render> lfml::Render for Hypermedia<T> {
    fn markup(&self) -> lfml::Markup {
        lfml::html! {
            @match self {
                Self::Document(t) => { (base(t)) }
                Self::Fragment(t) => { (t) }
            }
        }
    }
}

lfml::template!(pub index(todos: &[Todo]) {
    ."" data-todo-app {
        .todo-titlebar {
            h1 { "TODO" }
            ."" {
                p { "# Todos: " span { (todos.len()) } }
            }
        }
        details .data-todo-table {
            summary { h2 { "Name" } }
            ol data-todo-list {
                @for todo in todos { (todo.render_display()) }
            }
        }
        h3 { "Add a new row" }
        form
            hx-post="/todos/new"
            hx-target="[data-todo-list]"
            hx-swap="beforeend"
        {
            label for="name" {
                span { "Name " }
                input name="name" type="text";
            }
            button { "\u{2795}" }
        }
    }
});
