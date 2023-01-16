export interface Color {
  tint: {
    1: string;
    [variant: number]: string;
  };
  base: string;
  shade: {
    1: string;
    [variant: number]: string;
  };
}

export const blue: Color = {
  tint: {
    1: '#00aff5',
    2: '#67c8ff',
    3: '#aadeff',
    4: '#e3f4ff',
  },
  base: '#0094d0',
  shade: {
    1: '#0077a8',
    2: '#005b81',
    3: '#003f5c',
    4: '#002437',
  },
};

export const red: Color = {
  tint: {
    1: '#f5716b',
    2: '#fa9e96',
    3: '#fcc6c1',
    4: '#feedeb',
  },
  base: '#e44646',
  shade: {
    1: '#bd3033',
    2: '#932224',
    3: '#6a1517',
    4: '#41090a',
  },
};

export const gray: Color = {
  tint: {
    1: '#a8a8a8',
    2: '#c0c0c0',
    3: '#d8d8d8',
    4: '#f1f1f1',
  },
  base: '#919191',
  shade: {
    1: '#747474',
    2: '#575757',
    3: '#3c3c3c',
    4: '#212121',
  },
};

export const green: Color = {
  tint: {
    1: '#1dd281',
    2: '#21ea90',
    3: '#5ffeaa',
    4: '#cfffe0',
  },
  base: '#1bba72',
  shade: {
    1: '#15945a',
    2: '#0e6f42',
    3: '#074b2c',
    4: '#022915',
  },
};

export const info = blue;
export const success = green;
export const error = red;
