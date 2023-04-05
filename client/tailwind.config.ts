module.exports = {
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {
        extend: {},
    },
    darkMode: "class",
    plugins: [
        require("@tailwindcss/typography"),
        require("@tailwindcss/forms"),
    ],
}
