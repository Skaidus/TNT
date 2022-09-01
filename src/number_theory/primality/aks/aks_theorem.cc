#include "tnt-lib/include/aks_theorem.h"

using namespace std;
using namespace NTL;

bool aks_theorem(unsigned int n, unsigned int r, unsigned int s){
        ZZ_p::init(to_ZZ(n));
        ZZ_pX f(r, 1); f -= 1;
        const ZZ_pXModulus pf(f);

        ZZ_pX rPoly(1, 1);				// x
        PowerMod(rPoly, rPoly, n, pf);	// x^n
        unsigned int a;
        for (a = 1; a <= s; ++a)
        {
            ZZ_pX lPoly(1, 1);	// x
            lPoly -= a;			// x - a
            PowerMod(lPoly, lPoly, n, pf);	// (x - a)^n
            lPoly += a;			// (x - a)^n + a
            if (lPoly != rPoly)
            {
                return false;
            }
        }
        return true;
}
