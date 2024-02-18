import colorDefinition from './colors.json';

export type Color = {
  tint: Record<string, string>;
  shade: Record<string, string>;
  DEFAULT: string;
  base: string;
  [custom: `${number}`]: string;
};

const colors = Object.fromEntries(
  colorDefinition.hues.map(hue => {
    let colors: Partial<Color> = Object.fromEntries(
      hue.colors.map((color, index) => [colorDefinition.tones[index], color])
    );

    colors.DEFAULT = colors['500'];
    colors.base = colors['500'];

    colors.tint = Object.fromEntries(
      ['400', '300', '200', '100', '50'].map((tone, index) => [
        index + 1,
        colors[tone as keyof Color],
      ])
    ) as Record<string, string>;

    colors['shade'] = Object.fromEntries(
      ['600', '700', '800', '900'].map((tone, index) => [
        index + 1,
        colors[tone as keyof Color],
      ])
    ) as Record<string, string>;

    return [hue.name, colors as Color];
  })
);

export default colors;
