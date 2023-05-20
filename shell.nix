with import <nixpkgs> {};

stdenv.mkDerivation rec {
  name = "iced-env";
  buildInputs = [
    expat
    fontconfig
    freetype
    freetype.dev
    libGL
    pkgconfig
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];

  LD_LIBRARY_PATH = builtins.foldl'
    (a: b: "${a}:${b}/lib") "${vulkan-loader}/lib" buildInputs;
}
