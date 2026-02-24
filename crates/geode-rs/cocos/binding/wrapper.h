#ifndef BINDING_WRAPPER_H
#define BINDING_WRAPPER_H

#include <stdint.h>
#include <stddef.h>

typedef unsigned char GLubyte;
typedef int GLint;
typedef unsigned int GLuint;
typedef int GLenum;
typedef float GLfloat;
typedef int GLsizei;

namespace cocos2d {

class CCSize;

class CCPoint {
public:
    float x;
    float y;
};

class CCSize {
public:
    float width;
    float height;
};

class CCRect {
public:
    CCPoint origin;
    CCSize size;
};

typedef struct _ccColor3B {
    GLubyte r;
    GLubyte g;
    GLubyte b;
} ccColor3B;

typedef struct _ccColor4B {
    GLubyte r;
    GLubyte g;
    GLubyte b;
    GLubyte a;
} ccColor4B;

typedef struct _ccColor4F {
    GLfloat r;
    GLfloat g;
    GLfloat b;
    GLfloat a;
} ccColor4F;

typedef struct _ccHSVValue {
    float h;
    float s;
    float v;
    bool absoluteSaturation;
    bool absoluteBrightness;
} ccHSVValue;

class CCObject;
class CCNode;
class CCEvent;

typedef void (CCObject::*SEL_SCHEDULE)(float);
typedef void (CCObject::*SEL_CallFunc)();
typedef void (CCObject::*SEL_CallFuncN)(CCNode*);
typedef void (CCObject::*SEL_CallFuncND)(CCNode*, void*);
typedef void (CCObject::*SEL_CallFuncO)(CCObject*);
typedef void (CCObject::*SEL_MenuHandler)(CCObject*);
typedef void (CCObject::*SEL_EventHandler)(CCEvent*);
typedef int (CCObject::*SEL_Compare)(CCObject*);

typedef struct _ccTex2F {
    GLfloat u;
    GLfloat v;
} ccTex2F;

typedef struct _ccVertex2F {
    GLfloat x;
    GLfloat y;
} ccVertex2F;

typedef struct _ccVertex3F {
    GLfloat x;
    GLfloat y;
    GLfloat z;
} ccVertex3F;

typedef struct _ccColor3F {
    GLfloat r;
    GLfloat g;
    GLfloat b;
} ccColor3F;

typedef struct _ccV2F_C4B_T2F {
    ccVertex2F vertices;
    ccColor4B colors;
    ccTex2F texCoords;
} ccV2F_C4B_T2F;

typedef struct _ccV2F_C4F_T2F {
    ccVertex2F vertices;
    ccColor4F colors;
    ccTex2F texCoords;
} ccV2F_C4F_T2F;

typedef struct _ccV3F_C4B_T2F {
    ccVertex3F vertices;
    ccColor4B colors;
    ccTex2F texCoords;
} ccV3F_C4B_T2F;

typedef struct _ccBlendFunc {
    GLenum src;
    GLenum dst;
} ccBlendFunc;

}

enum enumKeyCodes {
    KEY_None = 0,
    KEY_A = 0x41,
    KEY_B = 0x42,
    KEY_C = 0x43,
    KEY_D = 0x44,
    KEY_E = 0x45,
    KEY_F = 0x46,
    KEY_G = 0x47,
    KEY_H = 0x48,
    KEY_I = 0x49,
    KEY_J = 0x4A,
    KEY_K = 0x4B,
    KEY_L = 0x4C,
    KEY_M = 0x4D,
    KEY_N = 0x4E,
    KEY_O = 0x4F,
    KEY_P = 0x50,
    KEY_Q = 0x51,
    KEY_R = 0x52,
    KEY_S = 0x53,
    KEY_T = 0x54,
    KEY_U = 0x55,
    KEY_V = 0x56,
    KEY_W = 0x57,
    KEY_X = 0x58,
    KEY_Y = 0x59,
    KEY_Z = 0x5A,
    KEY_0 = 0x30,
    KEY_1 = 0x31,
    KEY_2 = 0x32,
    KEY_3 = 0x33,
    KEY_4 = 0x34,
    KEY_5 = 0x35,
    KEY_6 = 0x36,
    KEY_7 = 0x37,
    KEY_8 = 0x38,
    KEY_9 = 0x39,
    KEY_F1 = 0x70,
    KEY_F2 = 0x71,
    KEY_F3 = 0x72,
    KEY_F4 = 0x73,
    KEY_F5 = 0x74,
    KEY_F6 = 0x75,
    KEY_F7 = 0x76,
    KEY_F8 = 0x77,
    KEY_F9 = 0x78,
    KEY_F10 = 0x79,
    KEY_F11 = 0x7A,
    KEY_F12 = 0x7B,
    KEY_Space = 0x20,
    KEY_Escape = 0x1B,
    KEY_Enter = 0x0D,
    KEY_Tab = 0x09,
    KEY_Backspace = 0x08,
    KEY_Insert = 0x2D,
    KEY_Delete = 0x2E,
    KEY_Home = 0x24,
    KEY_End = 0x23,
    KEY_PageUp = 0x21,
    KEY_PageDown = 0x22,
    KEY_Up = 0x26,
    KEY_Down = 0x28,
    KEY_Left = 0x25,
    KEY_Right = 0x27,
    KEY_Shift = 0x10,
    KEY_Control = 0x11,
    KEY_Alt = 0x12,
    KEY_CapsLock = 0x14,
    KEY_Comma = 0xBC,
    KEY_Period = 0xBE,
    KEY_Minus = 0xBD,
    KEY_Plus = 0xBB,
};

#endif
