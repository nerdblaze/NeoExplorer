/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors: {
        primarybackground: "var(--PrimaryBackground)",
        secondarybackground: "var(--SecondaryBackground)",
        surfacebackground: "var(--SurfaceBackground)",
        
        primarytext: "var(--PrimaryText)",
        secondarytext: "var(--SecondaryText)",
        hinttext: "var(--HintText)",
        
        accentprimary: "var(--AccentPrimary)",
        accentsecondary: "var(--AccentSecondary)",
        accenterror: "var(--AccentError)",
        accentwarning: "var(--AccentWarning)",
        accentinfo: "var(--AccentInfo)",
        accentsuccess: "var(--AccentSuccess)",
        dividerline: "var(--DividerLine)",
      },
      fontSize: {
        '3xs': ["0.25rem","0.50rem"],
        '2xs': ["0.5rem","0.75rem"]
      },
    },
  },
  plugins: [],
};
