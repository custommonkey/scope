#version 140

uniform float     iGlobalTime;           // shader playback time (in seconds)
uniform sampler2D iChannel0;          // input channel. XX = 2D/Cube
uniform sampler2D iChannel1;          // input channel. XX = 2D/Cube

vec2 iResolution = vec2(1024, 768);

float noise(vec2 p)
{
    float sample = texture(iChannel1, vec2(1., 2. * cos(iGlobalTime)) * iGlobalTime * 8.0 + p * 1.0).x;
    sample *= sample;
    return sample;
}

float onOff(float a, float b, float c)
{
    return step(c, sin(iGlobalTime + a*cos(iGlobalTime*b)));
}

float ramp(float y, float start, float end)
{
    float inside = step(start,y) - step(end,y);
    float fact = (y-start)/(end-start)*inside;
    return (1.-fact) * inside;   
}

float stripes(vec2 uv)
{
    float noi = noise(uv*vec2(0.5,1.) + vec2(1.,3.));
    return ramp(mod(uv.y*4. + iGlobalTime/2.+sin(iGlobalTime + sin(iGlobalTime*0.63)),1.),0.5,0.6)*noi;
}

vec3 getVideo(vec2 uv)
{
    vec2 look = uv;
    float window = 1./(1.+20.*(look.y-mod(iGlobalTime/4.,1.))*(look.y-mod(iGlobalTime/4.,1.)));
    look.x = look.x + sin(look.y*10. + iGlobalTime)/50.*onOff(4.,4.,.3)*(1.+cos(iGlobalTime*80.))*window;
    float vShift = 0.4*onOff(2.,3.,.9)*(sin(iGlobalTime)*sin(iGlobalTime*20.) +
                                        (0.5 + 0.1*sin(iGlobalTime*200.)*cos(iGlobalTime)));
    look.y = mod(look.y + vShift, 1.);
    vec3 video = vec3(texture(iChannel0,look));
    return video;
}

vec2 screenDistort(vec2 uv)
{
    uv -= vec2(.5,.5);
    uv = uv*1.2*(1./1.2+2.*uv.x*uv.x*uv.y*uv.y);
    uv += vec2(.5,.5);
    return uv;
}

out vec4 color;

void main()
{
    
    vec2 uv = gl_FragCoord.xy / iResolution.xy;
    uv = screenDistort(uv);
    vec3 video = getVideo(uv);
    float vigAmt = 3.+.3*sin(iGlobalTime + 5.*cos(iGlobalTime*5.));
    float vignette = (1.-vigAmt*(uv.y-.5)*(uv.y-.5))*(1.-vigAmt*(uv.x-.5)*(uv.x-.5));
    
    video += stripes(uv);
    video += noise(uv*2.)/2.;
    video *= vignette;
    video *= (12.+mod(uv.y*30.+iGlobalTime,1.))/23.;
    
    color = vec4(video,1.0);
}