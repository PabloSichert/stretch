#include <cstdint>
#include <cstdlib>

enum class AlignContent {
  FlexStart,
  FlexEnd,
  Center,
  Stretch,
  SpaceBetween,
  SpaceAround,
};

enum class AlignItems {
  FlexStart,
  FlexEnd,
  Center,
  Baseline,
  Stretch,
};

enum class AlignSelf {
  Auto,
  FlexStart,
  FlexEnd,
  Center,
  Baseline,
  Stretch,
};

enum class Direction {
  Inherit,
  LTR,
  RTL,
};

enum class FlexDirection {
  Row,
  Column,
  RowReverse,
  ColumnReverse,
};

enum class FlexWrap {
  NoWrap,
  Wrap,
  WrapReverse,
};

enum class JustifyContent {
  FlexStart,
  FlexEnd,
  Center,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
};

enum class Overflow {
  Visible,
  Hidden,
  Scroll,
};

enum class PositionType {
  Relative,
  Absolute,
};

template<typename T>
struct Size {
  T width;
  T height;
};

template<typename T>
struct Point {
  T x;
  T y;
};

template<typename T>
struct Array {
  T *pointer;
  uintptr_t length;
  uintptr_t capacity;
};

struct LayoutNode {
  Size<float> size;
  Point<float> location;
  Array<LayoutNode> children;
};

struct Dimension {
  enum class Tag {
    Undefined,
    Auto,
    Points,
    Percent,
  };

  struct Points_Body {
    float _0;
  };

  struct Percent_Body {
    float _0;
  };

  Tag tag;
  union {
    Points_Body points;
    Percent_Body percent;
  };
};

template<typename T>
struct Rect {
  T start;
  T end;
  T top;
  T bottom;
};

struct Number {
  enum class Tag {
    Defined,
    Undefined,
  };

  struct Defined_Body {
    float _0;
  };

  Tag tag;
  union {
    Defined_Body defined;
  };
};

struct StyleNode {
  PositionType position_type;
  Direction direction;
  FlexDirection flex_direction;
  FlexWrap flex_wrap;
  Overflow overflow;
  AlignItems align_items;
  AlignSelf align_self;
  AlignContent align_content;
  JustifyContent justify_content;
  Rect<Dimension> position;
  Rect<Dimension> margin;
  Rect<Dimension> padding;
  Rect<Dimension> border;
  float flex_grow;
  float flex_shrink;
  Dimension flex_basis;
  Size<Dimension> size;
  Size<Dimension> min_size;
  Size<Dimension> max_size;
  Number aspect_ratio;
  Array<StyleNode> children;
};

extern "C" {

void cleanup(const LayoutNode *layout);

LayoutNode unsafe_compute(const StyleNode *root);

} // extern "C"
