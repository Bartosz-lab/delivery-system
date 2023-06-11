import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { BehaviorSubject, Observable } from 'rxjs';
import { map } from 'rxjs/operators';
import { JwtHelperService } from '@auth0/angular-jwt';

import { environment } from 'src/environments/environment';
import { User } from './models/user';
import { isPartnerUser } from './models/role';

const httpOptions = {
  headers: new HttpHeaders({ 'Content-Type': 'application/json' }),
  withCredentials: true //this is required so that Angular returns the Cookies received from the server. The server sends cookies in Set-Cookie header. Without this, Angular will ignore the Set-Cookie header
};

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private userSubject: BehaviorSubject<User | null>;
  private user: Observable<User | null>;
  private jwthelper: JwtHelperService

  constructor(
    private router: Router,
    private http: HttpClient,
  ) {
    this.jwthelper = new JwtHelperService();
    this.userSubject = new BehaviorSubject(JSON.parse(sessionStorage.getItem('user')!));
    this.user = this.userSubject.asObservable();
  }

  login(username: string, password: string) {
    return this.http.post<any>(`${environment.apiUrl}/auth/login`, { "login": username, "password": password }, httpOptions)
      .pipe(map(response => {
        // store user details in local storage to keep user logged in between page refreshes
        const user = this.jwthelper.decodeToken(response['token']).user_info
        sessionStorage.setItem('user', JSON.stringify(user));
        this.userSubject.next(user);
        return response;
      }));
  }

  logout() {
    this.http.post<any>(`${environment.apiUrl}/auth/logout`, null, httpOptions).subscribe()
    sessionStorage.removeItem('user');
    this.userSubject.next(null);
    this.router.navigate(['/login']);
  }

  is_logged() {
    if (this.userSubject.value) {
      return true;
    }
    return false;
  }

  is_admin() {
    if (this.userSubject.value) {
      return this.userSubject.value.roles.includes("Admin");
    }
    return false;
  }

  is_courier() {
    if (this.userSubject.value) {
      return this.userSubject.value.roles.includes("Courier");
    }
    return false;
  }

  is_partner() {
    if (this.userSubject.value) {
      for (let role of this.userSubject.value.roles) {
        if (!(typeof role == 'string') && isPartnerUser(role)) {
          return true
        }
      }
    }
    return false;
  }

  get_parner_id() {
    if (this.userSubject.value) {
      for (let role of this.userSubject.value.roles) {
        if (!(typeof role == 'string') && isPartnerUser(role)) {
          return role.PartnerUser
        }
      }
    }
    return null;
  }
}
