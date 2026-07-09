using System.Runtime.InteropServices;

namespace Flappy;

/// <summary>
/// Mirror of the Rust <c>EngineApi</c> struct: a table of native function
/// pointers into the engine. Field order and signatures MUST stay in lockstep
/// with <c>flappy/src/ffi.rs</c>. All pointers use the C (cdecl) convention.
/// </summary>
[StructLayout(LayoutKind.Sequential)]
public unsafe struct EngineApi
{
    public delegate* unmanaged[Cdecl]<float> ScreenWidth;
    public delegate* unmanaged[Cdecl]<float> ScreenHeight;
    public delegate* unmanaged[Cdecl]<double> GetTime;

    public delegate* unmanaged[Cdecl]<int, int> KeyPressed;
    public delegate* unmanaged[Cdecl]<int> MouseLeftPressed;

    public delegate* unmanaged[Cdecl]<uint, void> ClearBackground;
    public delegate* unmanaged[Cdecl]<float, float, float, float, uint, void> DrawRectangle;
    public delegate* unmanaged[Cdecl]<float, float, float, float, float, uint, void> DrawRectangleLines;
    public delegate* unmanaged[Cdecl]<float, float, float, uint, void> DrawCircle;
    public delegate* unmanaged[Cdecl]<float, float, float, float, uint, void> DrawCircleLines;
    public delegate* unmanaged[Cdecl]<float, float, float, float, float, uint, void> DrawLine;
    public delegate* unmanaged[Cdecl]<float, float, float, float, float, float, uint, void> DrawTriangle;
    public delegate* unmanaged[Cdecl]<byte*, float, float, float, uint, void> DrawText;
    public delegate* unmanaged[Cdecl]<byte*, float, float> MeasureTextWidth;

    public delegate* unmanaged[Cdecl]<float, float, float> RandRange;
    public delegate* unmanaged[Cdecl]<void> Quit;
}
