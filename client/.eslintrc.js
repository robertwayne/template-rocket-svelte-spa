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
                "@typescript-eslint/array-type": [
                    "error",
                    {
                        default: "generic",
                    },
                ],
                "no-undef": "off",
            },
        },
        {
            files: ["*.ts"],
            extends: [
                "eslint:recommended",
                "plugin:@typescript-eslint/recommended",
            ],
            rules: {
                "@typescript-eslint/array-type": [
                    "error",
                    {
                        default: "generic",
                    },
                ],
                "no-undef": "off",
            },
        },
    ],
    rules: {},
    settings: {
        "svelte3/typescript": () => require("typescript"),
    },
}
