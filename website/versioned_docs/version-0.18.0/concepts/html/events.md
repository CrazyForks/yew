---
title: "Events"
---

## Introduction

Yew integrates with the [`web-sys`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate and
uses the events from that crate. The [table below](#event-types) lists all of the `web-sys`
events that are accepted in the `html!` macro.

You can still add a [`Callback`](../components/callbacks.md) for an event that is not listed in the table
below, see [Manual event listener](#manual-event-listener).

## Event Types

:::tip
All the event types mentioned in the following table are re-exported under `yew::events`.
Using the types from `yew::events` makes it easier to ensure version compatibility than
if you were to manually include `web-sys` as a dependency in your crate because you won't
end up using a version which conflicts with the version that Yew specifies.
:::

The event listener name is the expected name when adding an event `Callback` in the `html` macro:

```rust
use yew::{html, Callback};

html! {
    <button onclick={Callback::from(|_| ())}>
    //      ^^^^^^^ event listener name
        { "Click me!" }
    </button>
};
```

The event name is the listener without the "on" prefix, therefore, the `onclick` event listener
listens for `click` events.

| Event listener name         | `web_sys` Event Type                                                                  | `stdweb` Event Type                                                                                           |
| --------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `onabort`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [ResourceAbortEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ResourceAbortEvent.html)           |
| `onauxclick`                | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [AuxClickEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.AuxClickEvent.html)                     |
| `onblur`                    | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html)           | [BlurEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.BlurEvent.html)                             |
| `oncancel`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `oncanplay`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `oncanplaythrough`          | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onchange`                  | [ChangeData](https://docs.rs/yew/latest/yew/events/enum.ChangeData.html)              | [ChangeData](https://docs.rs/yew-stdweb/latest/yew_stdweb/events/enum.ChangeData.html)                        |
| `onclick`                   | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [ClickEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ClickEvent.html)                           |
| `onclose`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `oncontextmenu`             | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [ContextMenuEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ContextMenuEvent.html)               |
| `oncuechange`               | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `ondblclick`                | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [DoubleClickEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DoubleClickEvent.html)               |
| `ondrag`                    | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragEvent.html)                             |
| `ondragend`                 | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragEndEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragEndEvent.html)                       |
| `ondragenter`               | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragEnterEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragEnterEvent.html)                   |
| `ondragexit`                | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragExitEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragExitEvent.html)                     |
| `ondragleave`               | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.htmk)             | [DragLeaveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragLeaveEvent.html)                   |
| `ondragover`                | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragOverEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragOverEvent.html)                     |
| `ondragstart`               | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragStartEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragStartEvent.html)                   |
| `ondrop`                    | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragDropEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragDropEvent.html)                     |
| `ondurationchange`          | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onemptied`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onended`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onerror`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [ResourceErrorEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ResourceErrorEvent.html)           |
| `onfocus`                   | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html)           | [FocusEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.FocusEvent.html)                           |
| `onformdata`                | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `oninput`                   | [InputData](https://docs.rs/yew/latest/yew/events/struct.InputData.html)              | [InputData](https://docs.rs/yew-stdweb/latest/yew_stdweb/events/struct.InputData.html)                        |
| `oninvalid`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onkeydown`                 | [KeyboardEvent](https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html)     | [KeyDownEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.KeyDownEvent.html)                       |
| `onkeypress`                | [KeyboardEvent](https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html)     | [KeyPressEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.KeyPressEvent.html)                     |
| `onkeyup`                   | [KeyboardEvent](https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html)     | [KeyUpEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.KeyUpEvent.html)                           |
| `onload`                    | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [ResourceLoadEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ResourceLoadEvent.html)             |
| `onloadeddata`              | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onloadedmetadata`          | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onloadstart`               | [ProgressEvent](https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html)     | [LoadStartEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.LoadStartEvent.html)                   |
| `onmousedown`               | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseDownEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseDownEvent.html)                   |
| `onmouseenter`              | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseEnterEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseEnterEvent.html)                 |
| `onmouseleave`              | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseLeaveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseLeaveEvent.html)                 |
| `onmousemove`               | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseMoveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseMoveEvent.html)                   |
| `onmouseout`                | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseOutEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseOutEvent.html)                     |
| `onmouseover`               | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseOverEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseOverEvent.html)                   |
| `onmouseup`                 | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseUpEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseUpEvent.html)                       |
| `onpause`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onplay`                    | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onplaying`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onprogress`                | [ProgressEvent](https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html)     | [ProgressEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ProgressEvent.html)                     |
| `onratechange`              | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onreset`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onresize`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [ResizeEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ResizeEvent.html)                         |
| `onscroll`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [ScrollEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ScrollEvent.html)                         |
| `onsecuritypolicyviolation` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onseeked`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onseeking`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onselect`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onslotchange`              | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [SlotChangeEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.SlotChangeEvent.html)                 |
| `onstalled`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onsubmit`                  | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html)           | [SubmitEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.SubmitEvent.html)                         |
| `onsuspend`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `ontimeupdate`              | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `ontoggle`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onvolumechange`            | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onwaiting`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onwheel`                   | [WheelEvent](https://docs.rs/web-sys/latest/web_sys/struct.WheelEvent.html)           | [MouseWheelEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseWheelEvent.html)                 |
| `oncopy`                    | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `oncut`                     | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onpaste`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onanimationcancel`         | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html)   | Unsupported                                                                                                   |
| `onanimationend`            | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html)   | Unsupported                                                                                                   |
| `onanimationiteration`      | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html)   | Unsupported                                                                                                   |
| `onanimationstart`          | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html)   | Unsupported                                                                                                   |
| `ongotpointercapture`       | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [GotPointerCaptureEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.GotPointerCaptureEvent.html)   |
| `onloadend`                 | [ProgressEvent](https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html)     | [LoadEndEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.LoadEndEvent.html)                       |
| `onlostpointercapture`      | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [LostPointerCaptureEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.LostPointerCaptureEvent.html) |
| `onpointercancel`           | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerCancelEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerCancelEvent.html)           |
| `onpointerdown`             | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerDownEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerDownEvent.html)               |
| `onpointerenter`            | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerEnterEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerEnterEvent.html)             |
| `onpointerleave`            | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerLeaveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerLeaveEvent.html)             |
| `onpointerlockchange`       | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [PointerLockChangeEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerLockChangeEvent.html)   |
| `onpointerlockerror`        | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [PointerLockErrorEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerLockErrorEvent.html)     |
| `onpointermove`             | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerMoveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerMoveEvent.html)               |
| `onpointerout`              | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerOutEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerOutEvent.html)                 |
| `onpointerover`             | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerOverEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerOverEvent.html)               |
| `onpointerup`               | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerUpEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerUpEvent.html)                   |
| `onselectionchange`         | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [SelectionChangeEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.SelectionChangeEvent.html)       |
| `onselectstart`             | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onshow`                    | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `ontouchcancel`             | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html)           | [TouchCancel](https://docs.rs/stdweb/latest/stdweb/web/event/struct.TouchCancel.html)                         |
| `ontouchend`                | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html)           | [TouchEnd](https://docs.rs/stdweb/latest/stdweb/web/event/struct.TouchEnd.html)                               |
| `ontouchmove`               | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html)           | [TouchMove](https://docs.rs/stdweb/latest/stdweb/web/event/struct.TouchMove.html)                             |
| `ontouchstart`              | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html)           | [TouchStart](https://docs.rs/stdweb/latest/stdweb/web/event/struct.TouchStart.html)                           |
| `ontransitioncancel`        | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) | Unsupported                                                                                                   |
| `ontransitionend`           | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) | Unsupported                                                                                                   |
| `ontransitionrun`           | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) | Unsupported                                                                                                   |
| `ontransitionstart`         | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) | Unsupported                                                                                                   |

## `oninput` and `onchange`

:::note
In the next version of Yew `oninput` and `onchange` `Callback`s will accept `InputEvent` and `Event`
respectively.

Yew will be removing `InputData` and `ChangeData` as they were too restrictive and could panic
in certain circumstances.
:::

:::caution
You **must** apply the `Callback` to the target element even though the `InputEvent`/`Event`
bubbles up, the `InputData`/`ChangeData` is expecting the "target" to also be the "currentTarget"
(see the caution in [Typed event target](#typed-event-target) section for more).

```rust ,title="DO NOT DO THIS"
use yew::{html, ChangeData, Html, InputData};

enum Msg {
    InputValue(String),
}


fn view(&self) -> Html {

    let oninput = self.link.callback(|e: InputData| Msg::InputValue(e.value));
    let onchange = self.link.batch_callback(|e: ChangeData| {
        if let ChangeData::Value(value) = e {
            Some(Msg::InputValue(value))
        } else {
            None
        }
    });

    html! {
        <div
            // The `InputEvent` can bubble and then will read the text content
            // of the div as the value in `InputData` which is not what you'd
            // expect!
            //highlight-next-line
            oninput={oninput}
            // The `Event` can bubble and will cause a panic when it tries
            // to create the `ChangeData` enum.
            //highlight-next-line
            onchange={onchange}
        >
            { "hi" }
            <input type="text" />
        </div>
    }
}
```

:::

### `oninput` using InputData

Yew wraps the `InputEvent` in an `InputData` type, the `InputData` type also contains the current
`value` of the element on which the `oninput` handler is applied.

Yew does this by trying to cast the element into an `input` or `textarea` element then calling their
respective `value` getter method or it will get the text content of the element.

```rust
use yew::{html, Html, InputData};

enum Msg {
    InputValue(String),
}


fn view(&self) -> Html {

    let oninput = self.link.callback(|e: InputData| Msg::InputValue(e.value));

    html! {
        <div>
            <input oninput={oninput} />
        </div>
    }
}
```

If you want to get the value as a number (`f64`) then `InputData` does have the `event` field which is the
`web_sys::InputEvent` that you can use with the information provided in the
[Typed event target](#typed-event-target) section and then you can call
[`value_as_number`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlInputElement.html#method.value_as_number)
method on the target.

### `onchange` using ChangeData

The `ChangeData` type is an enum which has a variant for the three supported element types:

| Variant name | Variant data type            | Element type                                          |
| ------------ | ---------------------------- | ----------------------------------------------------- |
| `Files`      | `web_sys::FileList`          | `input` with type of `file`                           |
| `Select`     | `web_sys::HtmlSelectElement` | `select` element                                      |
| `Value`      | `String`                     | `textarea` or `input` with any type other than `file` |

If `onchange` is used on any other element then the application **will panic** when Yew tries to
convert the `Event` into `ChangeData`.

#### Files

When a user has made a change to the files selected by an `input` element with the type `file`.

```rust
use yew::{html, Html, ChangeData};

fn view(&self) -> Html {

    let onchange = self.link.batch_callback(|e| {
        if let ChangeData::Files(files) = e {
            // do something with web_sys::FileList
        } else {
            None
        }
    });

    html! {
        <div>
            <input onchange={onchange} type="file" />
        </div>
    }
}
```

:::tip
Use `batch_callback` when you want to conditionally return a value from a `Callback`.
:::

_see [file_upload](https://github.com/yewstack/yew/tree/v0.18/examples/file_upload) for a
full example._

#### Select

When a user has made a change to the `select` element.

```rust
use yew::{html, Html, ChangeData};

fn view(&self) -> Html {

    let onchange = self.link.batch_callback(|e| {
        if let ChangeData::Select(select) = e {
            // do something with web_sys::HtmlSelectElement
        } else {
            None
        }
    });

    html! {
        <div>
            <select onchange={onchange}>
                <option value="English">{ "Hello!" }</option>
                <option value="French">{ "Bonjour!" }</option>
                <option value="German">{ "Guten Tag!" }</option>
            </select>
        </div>
    }
}
```

#### Value

When a user has made a change to a `textarea` or `input` element with any type other than `file`.

```rust
use yew::{html, Html, ChangeData};

fn view(&self) -> Html {

    let onchange = self.link.batch_callback(|e| {
        if let ChangeData::Value(value) = e {
            // do something with the String value
        } else {
            None
        }
    });

    html! {
        <div>
            <textarea onchange={onchange} />
        </div>
    }
}
```

_see [crm](https://github.com/yewstack/yew/tree/v0.18/examples/crm) for a
full example._

## Typed event target

:::caution
In this section **target ([Event.target](https://developer.mozilla.org/en-US/docs/Web/API/Event/target))**
is always referring to the element at which the event was dispatched from.

This will **not** always be the element at which the `Callback` is placed, that is the
[Event.currentTarget](https://developer.mozilla.org/en-US/docs/Web/API/Event/currentTarget)
:::

In event `Callback`s you may want to get the target of that event. For example, on the
`input` event you may want to update some internal state.

In Yew getting the target element in the correct type can be done in a couple of ways and we will go through
them here. Calling [`web_sys::Event::target`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target)
on an event returns an optional [`web_sys::EventTarget`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html)
type, which might not seem very useful when you want to know the value of your input element.

In all the approaches below we are going to tackle the same problem, so it's clear where the approach
differs opposed to the problem at hand.

**The Problem:**

We have an `oninput` `Callback` on my `<input>` element and each time it is invoked we want to send
an [update](../components#update) `Msg` to our component. We really want to get the value as a
number or `f64` for rust.

Our `Msg` enum looks like this:

```rust
pub enum Msg {
    InputValue(f64),
}
```

### Using `JsCast`

The [`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/index.html) crate has
a useful trait; [`JsCast`](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html)
which allows us to hop and skip our way to the type we want, as long as it implements `JsCast`. We can
do this cautiously, which involves some runtime checks and failure types like `Option` and `Result`,
or we can do it dangerously.

Enough talk, more code:

```toml title="Cargo.toml"
[dependencies]
# need wasm-bindgen for JsCast
wasm-bindgen = "0.2"
```

```rust
//highlight-next-line
use wasm_bindgen::JsCast;
use yew::{
    html,
    web_sys::{EventTarget, HtmlInputElement},
    Component, ComponentLink, Html, InputData,
};

pub struct Comp {
    link: ComponentLink<Self>,
}

pub enum Msg {
    InputValue(f64),
}

impl Component for Comp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        let Msg::InputValue(value) = msg;
        // do something with value
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let link = &self.link;

        // Use batch_callback so if something unexpected happens we can return
        // None and do nothing
        let on_cautious_change = link.batch_callback(|e: InputData| {
            let e = e.event;
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            //highlight-next-line
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| Msg::InputValue(input.value_as_number()))
        });

        let on_dangerous_change = link.callback(|e: InputData| {
            let e = e.event;
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            // You must KNOW target is a HtmlInputElement, otherwise
            // the call to value would be Undefined Behaviour (UB).
            //highlight-start
            Msg::InputValue(
                target
                    .unchecked_into::<HtmlInputElement>()
                    .value_as_number(),
            )
            //highlight-end
        });

        html! {
            <>
                <label for="cautious-input">
                    { "My cautious input:" }
                    <input oninput={on_cautious_change}
                        id="cautious-input"
                        type="text"
                    />
                </label>
                <label for="dangerous-input">
                    { "My dangerous input:" }
                    <input oninput={on_dangerous_change}
                        id="dangerous-input"
                        type="text"
                    />
                </label>
            </>
        }
    }
}
```

:::tip
Use `batch_callback` when you want to conditionally return a value from a `Callback`.
:::

The methods from `JsCast` are [`dyn_into`](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into)
and [`unchecked_into`](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.unchecked_into)
and you can probably see, they allowed
us to go from `EventTarget` to [`HtmlInputElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlInputElement.html).
The `dyn_into` method is cautious because at
runtime it will check whether the type is actually a `HtmlInputElement` and if not return an
`Err(JsValue)`, the [`JsValue`](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html)
is a catch-all type and is essentially giving you back the object to try again.

At this point you might be thinking... when is the dangerous version ok to use? In the case above it
is safe<sup>1</sup> as we've set the `Callback` on to an element with no children so the target can
only be that same element.

_<sup>1</sup> As safe as anything can be when JS land is involved._

### Using `NodeRef`

[`NodeRef`](../components/refs.md) can be used instead of querying the event given to a `Callback`.

```rust
//highlight-next-line
use yew::{html, web_sys::HtmlInputElement, Component, ComponentLink, Html, NodeRef};

pub struct Comp {
    link: ComponentLink<Self>,
    //highlight-next-line
    my_input: NodeRef,
}

pub enum Msg {
    InputValue(f64),
}

impl Component for Comp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            //highlight-next-line
            my_input: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        let Msg::InputValue(value) = msg;
        // do something with value
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let my_input_ref = self.my_input.clone();

        let oninput = self.link.batch_callback(move |_| {
            //highlight-next-line
            let input = my_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::InputValue(input.value_as_number()))
        });

        html! {
            <>
                <label for="my-input">
                    { "My input:" }
                    //highlight-next-line
                    <input ref={self.my_input.clone()}
                        oninput={oninput}
                        id="my-input"
                        type="text"
                    />
                </label>
            </>
        }
    }
}
```

Using `NodeRef`, you can ignore the event and use the `NodeRef::cast` method to get an
`Option<HtmlInputElement>` - this is optional as calling `cast` before the `NodeRef` has been
set, or when the type doesn't match will return `None`.

You might also see by using `NodeRef` we don't have to send the `f64` back in the
`Msg::InputValue` as we always have `my_input` in the component state - so we could do the following:

```rust
use yew::{html, web_sys::HtmlInputElement, Component, ComponentLink, Html, NodeRef};

pub struct Comp {
    link: ComponentLink<Self>,
    my_input: NodeRef,
}

pub enum Msg {
    // Signal the input element has changed
    //highlight-next-line
    InputChanged,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            my_input: NodeRef::default(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    //highlight-start
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged => {
                if let Some(input) = self.my_input.cast::<HtmlInputElement>() {
                    let value = input.value_as_number();
                    // do something with value

                    true
                } else {
                    false
                }
            }
        }
    }
    //highlight-end

    fn view(&self) -> Html {
        //highlight-next-line
        let oninput = self.link.callback(|_| Msg::InputChanged);

        html! {
            <label for="my-input">
                { "My input:" }
                <input ref={self.my_input.clone()}
                    oninput={oninput}
                    id="my-input"
                    type="text"
                />
            </label>
        }
    }
}
```

Which approach you take depends on your component and your preferences, there is no _blessed_ way
per se.

## Manual event listener

You may want to listen to an event that is not supported by Yew's `html` macro, see the
[supported events listed here](#event-types).

In order to add an event listener to one of elements manually we need the help of
[`NodeRef`](../components/refs) so that in the `rendered` method we can add a listener using the
[`web-sys`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/index.html) and
[wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/index.html) API.
We do this in `rendered` as this is the only time we can guarantee that the element exists in
the browser, Yew needs some time to create them after `view` is called.

The examples below are going to show adding listeners for the made-up `custard` event. All events
either unsupported by yew or custom can be represented as a
[`web_sys::Event`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Event.html). If you
need to access a specific method or field on a custom / unsupported event then you can use the
methods of [`JsCast`](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html)
in order to convert to the type required.

### Using `Closure` (verbose)

Using the `web-sys` and `wasm-bindgen` API's directly for this can be a bit painful.. so brace
yourself ([there is a more concise way thanks to `gloo`](#using-gloo-concise)).

```rust
use wasm_bindgen::{prelude::Closure, JsCast};
use yew::{
    html,
    web_sys::{Event, HtmlElement},
    Component, ComponentLink, Html, NodeRef,
};

pub struct Comp {
    link: ComponentLink<Self>,
    my_div: NodeRef,
    custard_listener: Option<Closure<dyn Fn(Event)>>,
}

pub enum Msg {
    Custard,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            my_div: NodeRef::default(),
            custard_listener: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Custard => {
                // do something about custard..
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div ref={self.my_div.clone()} id="my-div"></div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if !first_render {
            return;
        }

        if let Some(element) = self.my_div.cast::<HtmlElement>() {
            // Create your Callback as you normally would
            let oncustard = self.link.callback(|_: Event| Msg::Custard);

            // Create a Closure from a Box<dyn Fn> - this has to be 'static
            let listener =
                Closure::<dyn Fn(Event)>::wrap(
                    Box::new(move |e: Event| oncustard.emit(e))
                );
            element
                .add_event_listener_with_callback(
                    "custard",
                    listener.as_ref().unchecked_ref()
                )
                .unwrap();

            // Need to save listener in the component otherwise when the
            // event is fired it will try and call the listener that no longer
            // exists in memory!
            self.custard_listener = Some(listener);
        }
    }

    fn destroy(&mut self) {
        // All done with the component but need to remove
        // the event listener before the custard_listener memory
        // goes out of scope.
        if let (Some(element), Some(listener)) = (
            self.my_div.cast::<HtmlElement>(),
            self.custard_listener.take(),
        ) {
            element
                .remove_event_listener_with_callback(
                    "custard",
                    listener.as_ref().unchecked_ref()
                )
                .unwrap();
        }
    }
}
```

For more information on `Closures`, see
[The `wasm-bindgen` Guide](https://rustwasm.github.io/wasm-bindgen/examples/closures.html).

### Using `gloo` (concise)

The easier way is with `gloo`, more specifically [`gloo_events`](https://docs.rs/gloo-events/0.1.1/gloo_events/index.html)
which is an abstraction for `web-sys`, `wasm-bindgen`.

`gloo_events` has the `EventListener` type which can be used to create and store the
event listener.

```toml title="Cargo.toml"
[dependencies]
gloo-events = "0.1"
```

```rust
use yew::{
    html,
    web_sys::{Event, HtmlElement},
    Component, ComponentLink, Html, NodeRef,
};

use gloo::events::EventListener;

pub struct Comp {
    link: ComponentLink<Self>,
    my_div: NodeRef,
    custard_listener: Option<EventListener>,
}

pub enum Msg {
    Custard,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            my_div: NodeRef::default(),
            custard_listener: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Custard => {
                // do something about custard..
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div ref={self.my_div.clone()} id="my-div"></div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if !first_render {
            return;
        }

        if let Some(element) = self.my_div.cast::<HtmlElement>() {
            // Create your Callback as you normally would
            let oncustard = self.link.callback(|_: Event| Msg::Custard);

            let listener =
                EventListener::new(
                    &element,
                    "custard",
                    move |e| oncustard.emit(e.clone())
                );

            self.custard_listener = Some(listener);
        }
    }
}
```

Notice that when using an `EventListener` you don't need to do anything when the
component is about to be destroyed as the `EventListener` has a `drop` implementation
which will remove the event listener from the element.

For more information on `EventListener`, see the
[gloo_events docs.rs](https://docs.rs/gloo-events/0.1.1/gloo_events/struct.EventListener.html).