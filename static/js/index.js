import { Sortable } from './vendor/sortable.esm.js'

document.querySelectorAll("[data-todo-list]")
    .forEach((el) => {
        var sortableInst = Sortable.create(el, {});
    })
