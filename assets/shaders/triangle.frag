#version 330 core

out vec4 FragColor;

in vec4 Color;
in vec2 TexCoord;

uniform float aMixValue;
uniform sampler2D aTexture1;
uniform sampler2D aTexture2;

void main() {
  FragColor = mix(texture(aTexture1, TexCoord), texture(aTexture2, TexCoord), aMixValue );
}
