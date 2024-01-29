#include "macroquest/include/eqlib.h"
#include "macroquest/src/ffi/mod.rs.h"

namespace mqrust
{
    namespace eqlib
    {
        rust::Str EQGroundItem::name() const
        {
            return this->Name;
        }
    }

}
