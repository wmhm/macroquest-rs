#include "macroquest/include/eqlib.h"
#include "macroquest/src/ffi/mod.rs.h"

namespace mqrust
{
    namespace eqlib
    {
        rust::Str PlayerClient::name() const { return this->Name; }

        rust::Str EQGroundItem::name() const { return this->Name; }
    }

}
