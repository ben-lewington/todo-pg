use crate::Todo;

lfml::template!(pub base(inner: impl lfml::Render) {
    (lfml::DOCTYPE)
    html .js-disabled lang="en" {
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link href="/static/css/index.css" rel="stylesheet";
            link href="/static/favicon.ico" rel="icon" type="image/x-icon";
            script src="/static/js/index.js" defer {}
            script src="/static/thirdparty/htmx.min.js" type="module" defer {}
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
    h1 {
        "TODO"
    }
    ."" data-todos {
        h2 { "Name" }
        ol data-todo-list {
            @for todo in todos { (todo.render_display()) }
        }
        form
            hx-post="/todos/new"
            hx-target="[data-todo-list]"
            hx-swap="beforeend"
        {
            label for="name" {
                "Name: "
            }
            input name="name" type="text";
            button { "+" }
        }
    }
});
