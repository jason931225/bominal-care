// 식사 배달 서비스 — Meal Delivery Services
// Browse and order healthy meal options for seniors

'use client';

import { useState } from 'react';
import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

type DietType = '일반' | '저염' | '당뇨식' | '연화식';

interface MealOption {
  id: string;
  name: string;
  provider: string;
  price: number;
  days: number;
  mealsPerDay: number;
  dietTypes: DietType[];
  description: string;
  ingredients: string;
  subsidy: boolean;
  popular: boolean;
}

const MEAL_OPTIONS: MealOption[] = [
  {
    id: 'meal-1',
    name: '건강한 한식 도시락',
    provider: '행복 급식센터',
    price: 5_000,
    days: 5,
    mealsPerDay: 1,
    dietTypes: ['일반', '저염', '당뇨식'],
    description: '신선한 제철 재료로 만든 균형 잡힌 한식 도시락 (반찬 3가지 포함)',
    ingredients: '잡곡밥, 된장국, 제철 나물 3종',
    subsidy: true,
    popular: true,
  },
  {
    id: 'meal-2',
    name: '연화식 식사 세트',
    provider: '부드러운 식탁',
    price: 7_000,
    days: 5,
    mealsPerDay: 1,
    dietTypes: ['연화식'],
    description: '씹기 불편한 어르신을 위한 연화 가공 식사 — 영양은 그대로',
    ingredients: '연화 잡곡밥, 국, 연화 반찬 3종',
    subsidy: true,
    popular: false,
  },
  {
    id: 'meal-3',
    name: '당뇨 관리 도시락',
    provider: '메디푸드',
    price: 8_500,
    days: 5,
    mealsPerDay: 1,
    dietTypes: ['당뇨식', '저염'],
    description: '혈당 지수(GI)를 낮춘 당뇨 전용 식단. 영양사 설계.',
    ingredients: '현미밥, 저당 반찬 4종',
    subsidy: false,
    popular: false,
  },
];

const DIET_FILTER_OPTIONS: DietType[] = ['일반', '저염', '당뇨식', '연화식'];

export default function MealsPage() {
  const [selectedDiet, setSelectedDiet] = useState<DietType | 'all'>('all');
  const [orderedMeal, setOrderedMeal] = useState<string | null>(null);

  const filtered = selectedDiet === 'all'
    ? MEAL_OPTIONS
    : MEAL_OPTIONS.filter((m) => m.dietTypes.includes(selectedDiet));

  return (
    <SeniorAppShell>
      <div className="page-content">
        <Link
          href="/services"
          className="inline-flex items-center gap-2 text-primary-600 font-medium text-senior-base mb-5 min-h-touch"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
          서비스로
        </Link>

        <h1 className="text-senior-2xl font-bold text-gray-900 mb-2">식사 배달 서비스 🍱</h1>
        <p className="text-senior-base text-gray-500 mb-5">노인 맞춤 건강 식단 · 매일 배달</p>

        {/* Subsidy notice */}
        <div className="bg-success-50 border border-success-300 rounded-2xl p-4 mb-5 flex items-start gap-3">
          <span className="text-2xl flex-shrink-0" aria-hidden="true">🎁</span>
          <div>
            <p className="text-senior-base font-bold text-success-700">급식 지원 대상자 안내</p>
            <p className="text-senior-sm text-success-700">기초생활수급자, 차상위계층은 최대 월 20식 무료 또는 할인 지원이 가능합니다. 주민센터에 문의하세요.</p>
          </div>
        </div>

        {/* Diet filter */}
        <div className="flex gap-2 mb-5 overflow-x-auto pb-1" role="group" aria-label="식단 종류 필터">
          <button
            onClick={() => setSelectedDiet('all')}
            className={`flex-shrink-0 px-4 py-2 rounded-xl text-senior-sm font-semibold border-2 min-h-touch transition-colors
              ${selectedDiet === 'all' ? 'bg-primary-600 border-primary-600 text-white' : 'bg-white border-gray-300 text-gray-700'}`}
          >
            전체
          </button>
          {DIET_FILTER_OPTIONS.map((diet) => (
            <button
              key={diet}
              onClick={() => setSelectedDiet(diet)}
              className={`flex-shrink-0 px-4 py-2 rounded-xl text-senior-sm font-semibold border-2 min-h-touch transition-colors
                ${selectedDiet === diet ? 'bg-primary-600 border-primary-600 text-white' : 'bg-white border-gray-300 text-gray-700'}`}
            >
              {diet}
            </button>
          ))}
        </div>

        {/* Meal cards */}
        <div className="space-y-4">
          {filtered.map((meal) => (
            <div key={meal.id} className="senior-card">
              {/* Header */}
              <div className="flex items-start justify-between mb-2">
                <div>
                  {meal.popular && (
                    <span className="bg-warning-100 text-warning-700 text-xs font-bold px-2 py-0.5 rounded-full mr-2">인기</span>
                  )}
                  {meal.subsidy && (
                    <span className="bg-success-50 text-success-700 text-xs font-bold px-2 py-0.5 rounded-full">지원 가능</span>
                  )}
                  <p className="text-senior-xl font-bold text-gray-900 mt-1">{meal.name}</p>
                  <p className="text-senior-sm text-gray-500">{meal.provider}</p>
                </div>
                <div className="text-right flex-shrink-0 ml-3">
                  <p className="text-senior-xl font-bold text-primary-700">{meal.price.toLocaleString('ko-KR')}원</p>
                  <p className="text-senior-sm text-gray-500">1식 기준</p>
                </div>
              </div>

              <p className="text-senior-base text-gray-700 mb-2">{meal.description}</p>
              <p className="text-senior-sm text-gray-500 mb-3">🥘 {meal.ingredients}</p>

              {/* Diet type badges */}
              <div className="flex gap-1.5 flex-wrap mb-4">
                {meal.dietTypes.map((diet) => (
                  <span key={diet} className="bg-gray-100 text-gray-600 text-senior-sm px-2.5 py-0.5 rounded-full font-medium">
                    {diet}
                  </span>
                ))}
              </div>

              {orderedMeal === meal.id ? (
                <div className="bg-success-50 rounded-xl p-3 text-center">
                  <p className="text-senior-base font-bold text-success-700">✅ 주문이 접수됐습니다!</p>
                  <p className="text-senior-sm text-success-600 mt-0.5">내일부터 매일 점심 배달 예정</p>
                </div>
              ) : (
                <button
                  onClick={() => setOrderedMeal(meal.id)}
                  className="senior-btn-primary w-full"
                >
                  주문하기
                </button>
              )}
            </div>
          ))}
        </div>

        {filtered.length === 0 && (
          <div className="text-center py-12 text-gray-400">
            <p className="text-senior-lg">해당 식단이 없습니다.</p>
          </div>
        )}
      </div>
    </SeniorAppShell>
  );
}
