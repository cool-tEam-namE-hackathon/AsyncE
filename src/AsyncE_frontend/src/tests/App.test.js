import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";
import App from "../App.vue";

describe("App", () => {
    it("renders as expected", () => {
        const root = document.createElement("div");
        root.id = "root";
        document.body.appendChild(root);
        mount(App, { attachTo: root });

        expect(root.querySelector("main")).toBeTruthy();
    });
});
