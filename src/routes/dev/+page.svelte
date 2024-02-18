<script>
  import Footer from "$lib/components/footer.svelte";
  import Highlight from "svelte-highlight";
  import shell from "svelte-highlight/languages/shell";
  import rust from "svelte-highlight/languages/rust";
  import github from "svelte-highlight/styles/github";
  import githubDark from "svelte-highlight/styles/github-dark";
  import { mode } from "mode-watcher";

  const mapCode = `use bevy::prelude::*;
use bevy_generative::map::{MapBundle, MapPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MapBundle::default());
}
`;
  const terrainCode = `use bevy::prelude::*;
use bevy_generative::terrain::{TerrainBundle, TerrainPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TerrainPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(TerrainBundle::default());
}
`;
  const planetExample = `use bevy::prelude::*;
use bevy_generative::planet::{PlanetBundle, PlanetPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlanetPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PlanetBundle::default());
}
`;
</script>

<svelte:head>
  <title>For Developers • Procedra</title>
  {#if $mode == "dark"}
    {@html githubDark}
  {:else}
    {@html github}
  {/if}
</svelte:head>

<section>
  <article class="prose mx-auto my-8 p-4 dark:prose-invert md:p-0">
    <h1
      class="animate-gradient bg-gradient-to-r bg-clip-text text-4xl font-extrabold text-transparent"
    >
      For Developers
    </h1>
    <p>
      Hey developers, welcome to the world of procedural generation with bevy_generative! Are you
      tired of being restricted by the limited customization options of our current web app? Or
      perhaps you're eager to integrate procedural generation directly into your Bevy game? Well,
      you're in luck because bevy_generative has got you covered! In this guide, we'll explore how
      to get started with this powerful plugin for the Bevy game engine. But before we dive in, make
      sure you have Rust and Cargo installed on your system.
    </p>
    <h2>What is bevy_generative?</h2>
    <p>
      bevy_generative is a plugin designed specifically for the Bevy engine, allowing you to perform
      real-time procedural generation of maps, textures, terrain, planets, and more! Whether you're
      creating dynamic landscapes, generating unique textures, or crafting immersive 3D
      environments, bevy_generative has got you covered.
    </p>
    <h2>Installation</h2>
    <p>
      To get started, you'll need to add bevy_generative to your Rust project. You can do this by
      running the following command in your terminal:
    </p>
    <Highlight language={shell} code="cargo add bevy_generative" />
    <h2>Examples</h2>
    <p>Let's dive into some examples to see bevy_generative in action!</p>
    <h3>Maps and Textures</h3>
    <p>
      Maps and textures are essential elements of any game world. With bevy_generative, you can
      dynamically generate maps and textures in real-time, allowing for endless variations and
      customization. In this example, we'll show you how to spawn a basic 2D camera and generate a
      map.
    </p>
    <Highlight language={rust} code={mapCode} />
    <h3>Terrain</h3>
    <p>
      Creating realistic terrain can breathe life into your game world. With bevy_generative, you
      can generate detailed terrain meshes with ease. In this example, we'll demonstrate how to
      spawn a 3D camera and generate terrain.
    </p>
    <Highlight language={rust} code={terrainCode} />
    <h3>Planets</h3>
    <p>
      Transport your players to distant worlds with procedurally generated planets. With
      bevy_generative, you can create stunning planetary landscapes on the fly. In this example,
      we'll illustrate how to spawn a 3D camera and generate a planet. Note that these are not
      highly detailed and are recommended for backdrops.
    </p>
    <Highlight language={rust} code={planetExample} />
    <h2>Source Code and API Documentation</h2>
    <p>
      The source code for bevy_generative can be found on <a
        href="https://github.com/manankarnik/procedra">Github</a
      >. The complete API documentation for bevy_generative, including all the customizable
      parameters, can be found <a href="https://docs.rs/bevy_generative">here</a>. Be sure to check
      it out for detailed information on how to fine-tune your procedural generation to suit your
      needs.
    </p>
    <h2>Bevy Compatibility</h2>
    <p>bevy_generative is compatible with Bevy 0.12.</p>
    <h2>Contributing</h2>
    <p>
      We're not accepting pull requests at this time, but we welcome issues, feature requests, and
      bug reports. Your feedback is valuable in improving bevy_generative for everyone!
    </p>
    <h2>License</h2>
    <p>
      bevy_generative is dual-licensed under the MIT License and Apache License, Version 2.0. You
      can choose the license that suits your needs best.<br /> Happy coding! 🚀
    </p>
    <p>~ Manan Karnik</p>
  </article>
</section>

<Footer />
