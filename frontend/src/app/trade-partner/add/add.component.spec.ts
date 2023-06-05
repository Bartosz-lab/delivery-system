import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TradePartnerAddComponent } from './add.component';

describe('TradePartnerAddComponent', () => {
  let component: TradePartnerAddComponent;
  let fixture: ComponentFixture<TradePartnerAddComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [TradePartnerAddComponent]
    });
    fixture = TestBed.createComponent(TradePartnerAddComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
