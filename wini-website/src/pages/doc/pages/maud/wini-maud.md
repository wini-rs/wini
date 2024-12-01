# Wini-maud

Maud is great, but it's not perfect in all the cases.

During the development of wini, I encountered 2 problems, that were best solved by forking maud. Here is what they are, and why there are.


## Markup

By default, the `html!` proc macro returns a `Markup` which is a `PreEscaped<String>`, which is more or less, just a wrapper around `String`.

This is pretty normal, but, it didn't fit a scenario: including stylesheets and javascript files.

When you declare a component for example, this component might depends on some stylesheets or some scripts. So, when you include it, you would like to include those in the response (else the result will lack in functionnality or style).

But, since you basically only return a `String`, you can't really pass this context to the parent function that called you.

<div class="note">

In theory you could have wrote a proc macro that takes the last expression of a function, creates a new struct from the `PreEscaped<String>`, but I don't think that this was a good idea since you could just do that in `html!`

</div>


## Components

The syntax of 

```css
div {
    [my_component]
}
```

is unique to `wini-maud`, and exists for the same reason as above.

Its this syntax that makes the heritance of stylesheet and scripts possible. Every component included with this syntax will have their still being transmitted by the parent `html!` that created it.
