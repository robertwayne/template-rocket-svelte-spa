module.exports = {
    env: {
        browser: true,
        node: true,
    },
    parser: "@typescript-eslint/parser",
    plugins: ["svelte3", "@typescript-eslint"],
    overrides: [
        {
            files: ["*.svelte"],
            processor: "svelte3/svelte3",
            extends: [
                "eslint:recommended",
                "plugin:@typescript-eslint/recommended",
            ],
            rules: {
                "no-undef": "off",
                "unused-imports/no-unused-imports": "error",
            },
        },
        {
            files: ["*.ts"],
            extends: ["plugin:@typescript-eslint/recommended"],
            rules: {
                "no-undef": "off",
            },
        },
    ],
    rules: {},
    settings: {
        "svelte3/typescript": () => require("typescript"),
    },
}
