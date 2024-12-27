import { Test, TestingModule } from '@nestjs/testing';
import { JwtUtilService } from './jwt-util.service';

describe('JwtUtilService', () => {
  let service: JwtUtilService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [JwtUtilService],
    }).compile();

    service = module.get<JwtUtilService>(JwtUtilService);
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });
});
