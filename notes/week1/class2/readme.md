### Class 2

---

We have covered `blinks` and `actions` on the class.

- [Actions](#action)
- [Blinks](#blink)

> [!NOTE]
> We can think Actions as a `backend`, while a blink (blockchain link) as a `frontend`.

### [Actions](#action)

We can think actions as a `backend`. Since they help us to send a rpc request to the related endpoint of the contract. For example, we can trigger a method if we know the related endpoint of that method.

> [!IMPORTANT]
> Actions return `transactions` on the solana blockchain to be previewed, signed, and sent across a various number of contexts.

Thanks to actions, ==users don't need to navigate to a different app== for signing or doing other transaction stuffs.

![action-structure](./images/actions-structure.avif)
_Actions Lifecycle structure from [Action-Structure](https://docs.dialect.to/documentation/actions/specification/execution-and-lifecycle)_

> [!WARNING]
> As you realize, there is no user feedback for signed transactions on blinks since no return statement hasn't designed yet!

### [Blink](#blink)

We can think blink as a `frontend`. Since actions return metadata like images, names or etc, Blink helps us to render standardized UI based on those metadata. UI consists of buttons, images, or whatever actions can return for blink.

> [!NOTE]
> Blockchain Links (Blink) turn any Solana action into a shareable, metadata-rich link.
> Blinks are like a `embedded dApp widgets. sort of.

**To be a Blink**, you need;

- solana-action uri (like --> `solana-action:https://actions.alice.com/donate`)
- actions.json (you put it to the root folder for mapping APIs)
- interstitial website using the `action` query parameter.

![Blink-Example](./images/blink-example.avif)
_Blink example from [Dialect](https://docs.dialect.to/documentation/actions/specification/execution-and-lifecycle#execution)_
