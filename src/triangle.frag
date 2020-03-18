#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec2 TexCoord;
} IN;

out vec4 Color;

uniform sampler2D tex;

void main()
{
    Color = texture(tex, IN.TexCoord);
}