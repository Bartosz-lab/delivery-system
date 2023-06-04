import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TradeParnerListComponent } from './list.component';

describe('TradeParnerListComponent', () => {
  let component: TradeParnerListComponent;
  let fixture: ComponentFixture<TradeParnerListComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [TradeParnerListComponent]
    });
    fixture = TestBed.createComponent(TradeParnerListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
