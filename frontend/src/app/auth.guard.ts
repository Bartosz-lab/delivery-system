import { Router, CanActivateFn } from '@angular/router';
import { inject } from '@angular/core';

import { AuthService } from './auth.service';
import { Role } from './models/role';

export const authGuard: CanActivateFn = (route, state) => {
  const authService: AuthService = inject(AuthService);
  const router: Router = inject(Router);

  const expected_role = route.data["role"];

  if (authService.is_logged()) {
    if ((expected_role == Role.Admin && authService.is_admin())
      || (expected_role == Role.Courier && authService.is_courier())
      || (expected_role == Role.PartnerUser && authService.is_partner())
      || expected_role == null) {
      return true;
    }
  }

  // not logged in so redirect to login page with the return url
  router.navigate(['/login'], { queryParams: { returnUrl: state.url } });
  return false;
};
