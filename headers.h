#include "core/msg.h"
#include "core/n64video.h"
#include "core/parallel.h"
// Contains a few GL init functions
// but also contains all of OpenGL (no thanks)
//#include "output/gl_core_3_3.h"
#include "output/gl_proc.h"
#include "output/screen.h"
#include "output/vdac.h"
#include "plugin/mupen64plus/gfx_m64p.h"
#include "plugin/mupen64plus/api/m64p_common.h"
#include "plugin/mupen64plus/api/m64p_config.h"
#include "plugin/mupen64plus/api/m64p_plugin.h"
#include "plugin/mupen64plus/api/m64p_types.h"
#include "plugin/mupen64plus/api/m64p_vidext.h"


// The stuff from gl_core_3_3.h that might be useful
enum ogl_LoadStatus
{
	ogl_LOAD_FAILED = 0,
	ogl_LOAD_SUCCEEDED = 1,
};

int ogl_LoadFunctions();

int ogl_GetMinorVersion(void);
int ogl_GetMajorVersion(void);
int ogl_IsVersionGEQ(int majorVersion, int minorVersion);

// Version stuff

#define CORE_BASE_NAME "angrylion's RDP Plus"
#define CORE_NAME "angrylion's RDP Plus"
#define CORE_SIMPLE_NAME "angrylion-plus"
