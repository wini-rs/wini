export {};

type TextFunction = {
    (): string;
    (t: string): void;
};

type CssFunction = {
    (key: string): string;
    (key: string, value: string): void;
};

declare global {
    // If it starts with a #, single element, else list
    function $<T extends string>(e: `#${string}`): HTMLElement;
    function $(e: string): NodeListOf<HTMLElement>;

    // biome-ignore lint/suspicious/noExplicitAny: console.log uses any.
    const log: (...data: any[]) => void;
    const $new: (
        k: string,
        p: { [keyof: string]: string },
        t?: string,
        c?: HTMLElement[],
    ) => HTMLElement;
    // biome-ignore lint/suspicious/noExplicitAny: the data returned can be of any type.
    const api: (method: string, url: string, data?: { [keyof: string]: string }) => any;
    // biome-ignore lint/suspicious/noExplicitAny: the data returned can be of any type.
    const get: (url: string) => any;
    // biome-ignore lint/suspicious/noExplicitAny: the data returned can be of any type.
    const post: (url: string, data?: { [keyof: string]: string }) => any;
    // biome-ignore lint/suspicious/noExplicitAny: the data returned can be of any type.
    const put: (url: string, data?: { [keyof: string]: string }) => any;
    // biome-ignore lint/suspicious/noExplicitAny: the data returned can be of any type.
    const patch: (url: string, data?: { [keyof: string]: string }) => any;
    // biome-ignore lint/suspicious/noExplicitAny: the data returned can be of any type.
    const del: (url: string) => any;

    interface NodeListOf<TNode extends Node> {
        each(callback: (node: TNode, index: number, nodeList: NodeListOf<TNode>) => void): void;
    }

    interface HTMLElement {
        on: (
            t: string,
            // biome-ignore lint/suspicious/noExplicitAny: the data returned can be of any type.
            l: (this: Document, ev: any) => any,
            o?: boolean | AddEventListenerOptions,
        ) => void;
        hasClass: (c: string) => boolean;
        addClass: (c: string) => void;
        rmClass: (c: string) => void;
        add: (e: (string | HTMLElement)[] | HTMLElement) => void;
        prep: (e: (string | HTMLElement)[] | HTMLElement) => void;
        text: TextFunction;
        css: CssFunction;
        $<T extends string>(e: `#${string}`): HTMLElement;
        $(e: string): NodeListOf<HTMLElement>;
    }
}
