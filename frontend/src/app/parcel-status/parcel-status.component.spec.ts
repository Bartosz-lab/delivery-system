import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ParcelStatusComponent } from './parcel-status.component';

describe('ParcelStatusComponent', () => {
  let component: ParcelStatusComponent;
  let fixture: ComponentFixture<ParcelStatusComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [ParcelStatusComponent]
    });
    fixture = TestBed.createComponent(ParcelStatusComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
