<script>
  import favicon from "$lib/assets/media/favicon.png";
  import { Navlink, NavLogo } from "$lib/nav/navbar";

  export let name = "";
  export const links = [];
</script>

<nav class="flex-nowrap items-center justify-center bg-transparent prose prose-invert min-w-full max-w-screen my-0 p-3">
  <NavLogo logo={favicon}>
    {name}
  </NavLogo>
  <div class="flex grow items-center justify-start p-3" id="main-menu">
    <Navlink links={links}/>
  </div>
  <div class="justify-end">
    <slot/>
  </div>
</nav>

<style>
  nav {
    display: flex;
    max-width: 100vw;
    min-width: 100%;
    position: sticky top;
    z-index: 50;
  }
</style>